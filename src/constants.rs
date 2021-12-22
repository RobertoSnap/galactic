use bevy::math::Vec3;

// spaceships
pub const PLAYER_SHIP1_BLUE: &str = "playerShip1_blue.png";

// map
pub const SPACE_BG_3: &str = "Space/space_bg_3.png";

// weapons
pub const LASER_BLUE01: &str = "Lasers/laserBlue01.png";

pub const LAYER_PLAYER: f32 = 2.;
pub const LAYER_MAP: f32 = 1.;

pub enum LAYERS {
    PLAYER,
    MAP,
}

pub static WORLD_STAGE: &str = "world";
pub struct Constants {}
impl Constants {
    pub fn get_layer_for(layer: LAYERS) -> f32 {
        match layer {
            LAYERS::PLAYER => 100.,
            LAYERS::MAP => 10.,
        }
    }
}
