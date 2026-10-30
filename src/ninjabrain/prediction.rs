use serde::Deserialize;

const OVERWORLD_BLOCKS_PER_NETHER_BLOCK: i64 = 8;
const NETHER_BLOCKS_PER_CHUNK: i32 = 2;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prediction {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub certainty: f64,
    pub overworld_distance: f64
}

impl Prediction {
    pub fn nether_x(&self) -> i32 {
        self.chunk_x * NETHER_BLOCKS_PER_CHUNK
    }

    pub fn nether_z(&self) -> i32 {
        self.chunk_z * NETHER_BLOCKS_PER_CHUNK
    }

    pub fn distance(&self) -> i64 {
        self.overworld_distance.floor() as i64
    }

    pub fn nether_distance(&self) -> i64 {
        self.distance()
            .div_euclid(OVERWORLD_BLOCKS_PER_NETHER_BLOCK)
    }
}
