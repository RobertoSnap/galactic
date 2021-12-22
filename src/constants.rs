use bevy::math::Vec3;

pub const PLAYER_SHIP1_BLUE: &str = "playerShip1_blue.png";
pub const SPACE_BG_3: &str = "Space/space_bg_3.png";

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
