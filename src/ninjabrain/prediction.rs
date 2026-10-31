use serde::Deserialize;

use super::PlayerPosition;

const OVERWORLD_BLOCKS_PER_NETHER_BLOCK: i64 = 8;
const NETHER_BLOCKS_PER_CHUNK: i32 = 2;
const BLOCKS_PER_CHUNK: i32 = 16;
const TARGET_OFFSET_IN_CHUNK: i32 = 4;
const HALF_TURN_DEGREES: f64 = 180.0;
const FULL_TURN_DEGREES: f64 = 360.0;

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

    pub fn overworld_x(&self) -> i32 {
        self.chunk_x * BLOCKS_PER_CHUNK + TARGET_OFFSET_IN_CHUNK
    }

    pub fn overworld_z(&self) -> i32 {
        self.chunk_z * BLOCKS_PER_CHUNK + TARGET_OFFSET_IN_CHUNK
    }

    pub fn turn_angle(&self, player: &PlayerPosition) -> f64 {
        let delta_x = f64::from(self.overworld_x()) - player.x_in_overworld;
        let delta_z = f64::from(self.overworld_z()) - player.z_in_overworld;
        let travel_angle = -delta_x.atan2(delta_z).to_degrees();
        let turn = (travel_angle - player.horizontal_angle) % FULL_TURN_DEGREES;

        if turn > HALF_TURN_DEGREES {
            turn - FULL_TURN_DEGREES
        } else if turn < -HALF_TURN_DEGREES {
            turn + FULL_TURN_DEGREES
        } else {
            turn
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn player(x: f64, z: f64, horizontal_angle: f64) -> PlayerPosition {
        PlayerPosition {
            x_in_overworld: x,
            z_in_overworld: z,
            horizontal_angle,
            is_in_overworld: true,
            is_in_nether: false
        }
    }

    fn prediction(chunk_x: i32, chunk_z: i32) -> Prediction {
        Prediction {
            chunk_x,
            chunk_z,
            certainty: 1.0,
            overworld_distance: 0.0
        }
    }

    #[test]
    fn faces_the_target_with_no_turn() {
        let target = prediction(0, 10);

        assert!(target.turn_angle(&player(4.0, 0.0, 0.0)).abs() < 1e-9);
    }

    #[test]
    fn turns_right_toward_west_and_left_toward_east() {
        let west = prediction(-10, 0);
        let east = prediction(10, 0);
        let facing_south = player(0.0, 4.0, 0.0);

        assert!((west.turn_angle(&facing_south) - 90.0).abs() < 1e-9);
        assert!((east.turn_angle(&facing_south) + 90.0).abs() < 1e-9);
    }

    #[test]
    fn wraps_into_a_half_turn() {
        let north = prediction(0, -10);
        let facing_west = player(4.0, 0.0, 90.0);
        let facing_east = player(4.0, 0.0, -90.0);

        assert!((north.turn_angle(&facing_west) - 90.0).abs() < 1e-9);
        assert!((north.turn_angle(&facing_east) + 90.0).abs() < 1e-9);
    }
}
