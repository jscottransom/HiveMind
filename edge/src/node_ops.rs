// Simulates sensor emission
use rand::Rng;

// Stable values
const GRID_FREQUENCY: f32 = 60.0;
const STATE_OF_CHARGE: f32 = 95.0;
const GRID_IMPORT: f32 = 0.0;
const GRID_EXPORT: f32 = 0.5;
const MAX_START_VARIANCE: i16 = 5;

#[derive(Debug)]
pub struct GridStats {
    variance: i16,
    frequency: f32,
    soc: f32,
    import: f32,
    export: f32
}

// This is more sensitive, so we'll lower the acceptability
const GRID_VARIANCE: i16 = 1;

// In Hz
fn gen_grid_frequency() -> f32 {
    let mut rng = rand::rng();
    let variance = rng.random_range(-GRID_VARIANCE..GRID_VARIANCE);

    GRID_FREQUENCY + variance as f32
}

// In %
fn gen_state_of_charge() -> f32 {
    let mut rng = rand::rng();
    let variance = rng.random_range(-MAX_START_VARIANCE..MAX_START_VARIANCE);

    STATE_OF_CHARGE + variance as f32
}

fn gen_grid_import() -> f32 {
    GRID_IMPORT
}

fn gen_grid_export() -> f32 {
    let mut rng = rand::rng();
    let variance = rng.random_range(-GRID_VARIANCE..GRID_VARIANCE);

    GRID_EXPORT
}

pub async fn run() -> Result<GridStats, std::io::Error> {

    let variance = GRID_VARIANCE;
    let freq = gen_grid_frequency();
    let soc = gen_state_of_charge();
    let import = gen_grid_import();
    let export = gen_grid_export();

    Ok(GridStats { variance: variance, frequency: freq, soc: soc, import: import, export: export })

}