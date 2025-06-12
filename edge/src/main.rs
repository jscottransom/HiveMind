// Coordinates sharing messages between nodes

use std::env::args;
use std::{error::Error, time::Duration};

use libp2p::{SwarmBuilder, kad};
use log::{error, info, warn};
use tracing_subscriber::EnvFilter;

use futures::stream::StreamExt;
use libp2p::kad::{
    Behaviour as KademliaBehavior, Event as KademliaEvent, store::MemoryStore as KademliaInMemory,
};
use libp2p::{
    Multiaddr, gossipsub, identity, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};

use libp2p::identify::{
    Behaviour as IdentifyBehavior, Config as IdentifyConfig, Event as IdentifyEvent,
};
use tokio::select;

mod node_ops;
use node_ops::run;


// Defines the Network and Discovery Behavior of a Node
// IdentifyBehavior allows for identifying new Peers
// GossipSub provides the distributed messaging Layer
#[derive(NetworkBehaviour)]
struct NodeBehavior {
    identify: IdentifyBehavior,
    kad: KademliaBehavior<KademliaInMemory>,
    gossip: gossipsub::Behaviour,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    // Secure key that we will use for the Peer ID
    let local_key = identity::Keypair::generate_ed25519();

    // Define the swarm behavior
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {
            // Each Behavior takes a config, and the config is passed to the behavior constructor

            // Establish the behavior for Kadamelia DHT
            // Default Memory store (in-memory), and default config settings
            let kad_cfg = kad::Config::default();
            let mem_store = kad::store::MemoryStore::new(key.public().to_peer_id());
            let kad = KademliaBehavior::with_config(key.public().to_peer_id(), mem_store, kad_cfg);

            // Establish behavior for Identify
            let key_clone = key.clone();
            let identify_config = IdentifyConfig::new("hive/1.0.0".to_string(), key_clone.public());
            let identify_behavior = IdentifyBehavior::new(identify_config);

            // Custom Gossip Behavior
            // Set a custom gossipsub configuration
            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .build()
                .map_err(std::io::Error::other)?; // Temporary hack because `build` does not return a proper `std::error::Error`.

            // Build a gossipsub network behaviour with default config
            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key_clone),
                gossipsub_config,
            )?;

            Ok(NodeBehavior {
                identify: identify_behavior,
                kad: kad,
                gossip: gossipsub,
            })
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    // Create a Gossipsub topic
    let topic = gossipsub::IdentTopic::new("hive");
    // subscribes to our topic
    swarm.behaviour_mut().gossip.subscribe(&topic)?;

    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port. QUIC uses UDP.

    // Listening and dialing dependent on setup as bootstrap node or follower
    if let Some(addr) = args().nth(1) {
        swarm.listen_on("/ip4/0.0.0.0/tcp/9000".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/udp/9001/quic-v1".parse()?)?;

        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        info!("Dialed address: {addr}");
    } else {
        info!("Act as bootstrap node");
        swarm.listen_on("/ip4/0.0.0.0/tcp/9000".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/udp/9001/quic-v1".parse()?)?;
    }

    loop {
        // select! {

            
        //     Ok(Some(node_stats)) = run() => {
        //         if let Err(e) = swarm
        //             .behaviour_mut().gossip
        //             .publish(topic.clone(), node_stats.as_bytes()) {
        //             println!("Publish error: {e:?}");
        //         }
        //     }

        //     Err(e) = run() => {
        //         // Log the error for observability.
        //         println!("Problem generating node stats: {e:?}");
        //     }
            
        //     event = swarm.select_next_some() => match event {
        //         SwarmEvent::Behaviour(NodeBehavior::Gossipsub(gossipsub::Event::Message {
        //             propagation_source: peer_id,
        //             message_id: id,
        //             message,
        //         })) => println!(
        //                 "Got message: '{}' with id: {id} from peer: {peer_id}",
        //                 String::from_utf8_lossy(&message.data),
        //             ),
        //         SwarmEvent::NewListenAddr { address, .. } => {
        //             println!("Local node is listening on {address}");
        //         }
        //         _ => {}
        //     }
        // }
    }
}
