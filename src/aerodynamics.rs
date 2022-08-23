use atm_refraction::air::us76_atmosphere;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::hud::*;
use crate::player::*;

// Physics and Aerodynamics

pub fn aerodynamical_forces(
    mut q_player_movement: Query<(&mut Velocity, &mut Transform), With<Player>>,
    mut set: ParamSet<(
        Query<&mut Text, With<TextVelocity>>,
        Query<&mut Text, With<TextRoll>>,
        Query<&mut Text, With<TextPitch>>,
        Query<&mut Text, With<TextYaw>>,
    )>,
) {
    const FORCE_DRAG: f32 = 0.0;
    const FORCE_THRUST: f32 = 2.0 * 74.53;
    const COEFF_LIFT: f32 = 0.37;
    const COEFF_S: f32 = 62.04;
    const MASS_PLANE: f32 = 20873.0;

    let (velocity, transform) = q_player_movement.iter_mut().last().unwrap();

    let (roll, pitch, mut yaw) = transform.rotation.to_euler(EulerRot::ZXY);
    yaw = 0.0;

    let velocity_fwd = (Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll) * velocity.linvel)
        .x
        .abs();

    let density = air_density(transform.translation.y as f64) as f32;
    let force_lift: f32 = COEFF_LIFT * density * velocity_fwd.powf(2.0) * COEFF_S / 2.0;

    let acc_vec = Vec3::new(
        0.0, //(FORCE_THRUST - FORCE_DRAG) / MASS_PLANE,
        0.0, //force_lift / MASS_PLANE - 9.81,
        0.0,
    );

    // Update debug text
    for mut text in &mut set.p0() {
        text.sections[1].value = format!("{velocity_fwd:.2}");
    }

    for mut text in &mut set.p1() {
        text.sections[1].value = format!("{roll:.2}");
    }

    for mut text in &mut set.p2() {
        text.sections[1].value = format!("{pitch:.2}");
    }

    for mut text in &mut set.p3() {
        text.sections[1].value = format!("{yaw:.2}");
    }
}

fn air_density(height: f64) -> f64 {
    let atmosphere = us76_atmosphere();
    const R: f64 = 287.0500676; // J/(kg*K)
    let p = atmosphere.pressure(height);
    let t = atmosphere.temperature(height);
    return p / (R * t);
}
