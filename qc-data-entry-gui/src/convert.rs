use common_math::rounding::Roundable;

use crate::constants::*;

// fn SigFig(val: f32, precision: int) f32 {
//     // half_away_from_zero
// }

pub fn sg_from_mass(mass: f32) -> f32 {
    (mass / SAMPLE_VOLUME).round_sf(4)
}

pub fn mass_from_sg(sg: f32) -> f32 {
    (sg * SAMPLE_VOLUME).round_sf(4)
}

pub fn sg_from_density(density: f32) -> f32 {
    (density / LB_PER_GAL).round_sf(4)
}

pub fn density_from_sg(sg: f32) -> f32 {
    (sg * LB_PER_GAL).round_sf(4)
}
