use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_terrain::prelude::*;
use bitflags::bitflags;

// Materials
#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    rigid_body: RigidBody,
    velocity: Velocity,
    external_force: ExternalForce,
    atmosphere_camera: AtmosphereCamera,
    terrain_view: TerrainView,
    #[bundle]
    camera_3d_bundle: Camera3dBundle,
}

pub fn new_player_bundle(postion: Vec3) -> PlayerBundle {
    return PlayerBundle {
        velocity: Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        },
        external_force: ExternalForce {
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
        },
        rigid_body: RigidBody::Dynamic,
        player: Player,
        atmosphere_camera: AtmosphereCamera(None),
        terrain_view: TerrainView,
        camera_3d_bundle: Camera3dBundle {
            camera: Camera {
                priority: 1,
                ..default()
            },
            transform: Transform::from_xyz(postion.x, postion.y, postion.z),
            ..default()
        },
    };
}

// Control

bitflags! {
    #[derive(Default)]
    pub struct PlayerActionFlags: u32 {
        const IDLE = 1 << 0;
        const ENGINE_UP = 1 << 1;
        const ENGINE_DOWN = 1 << 2;
        const ROLL_LEFT = 1 << 3;
        const ROLL_RIGHT = 1 << 4;
        const PITCH_UP = 1 << 5;
        const PITCH_DOWN = 1 << 6;
    }
}

pub fn keyboard_control(
    keys: Res<Input<KeyCode>>,
    player_movement_q: Query<(&mut Velocity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let mut player_action = PlayerActionFlags::IDLE;

    for key in keys.get_pressed() {
        if *key == KeyCode::Left {
            player_action |= PlayerActionFlags::ROLL_LEFT;
        }
        if *key == KeyCode::Right {
            player_action |= PlayerActionFlags::ROLL_RIGHT;
        }
        if *key == KeyCode::Up {
            player_action |= PlayerActionFlags::PITCH_UP;
        }
        if *key == KeyCode::Down {
            player_action |= PlayerActionFlags::PITCH_DOWN;
        }
        if *key == KeyCode::Space {
            player_action |= PlayerActionFlags::ENGINE_UP;
        }
        if *key == KeyCode::LControl {
            player_action |= PlayerActionFlags::ENGINE_DOWN;
        }
    }

    control_player(player_action, player_movement_q, time);
}

pub fn control_player(
    player_action: PlayerActionFlags,
    mut player_movement_q: Query<(&mut Velocity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let seconds = time.delta_seconds();

    for (mut velocity, mut transform) in player_movement_q.iter_mut() {
        *velocity = Velocity::linear(velocity.linvel);

        if player_action.contains(PlayerActionFlags::ROLL_LEFT) {
            velocity.angvel = transform.forward() * 0.5 * 3.14;
        }
        if player_action.contains(PlayerActionFlags::ROLL_RIGHT) {
            velocity.angvel = -transform.forward() * 0.5 * 3.14;
        }

        if player_action.contains(PlayerActionFlags::PITCH_UP) {
            velocity.angvel = transform.left() * 0.5 * 3.14;
            // *velocity = velocity.with_angular(AxisAngle::new(transform.left(), 0.5 * 3.14));
        }
        if player_action.contains(PlayerActionFlags::PITCH_DOWN) {
            velocity.angvel = -transform.left() * 0.5 * 3.14;
            // *velocity = velocity.with_angular(AxisAngle::new(transform.left(), -0.5 * 3.14));
        }

        // if player_action.contains(PlayerActionFlags::ENGINE_UP) {
        //     acceleration.linear.x += 10.0;
        // }
        // if player_action.contains(PlayerActionFlags::ENGINE_DOWN) {
        //     acceleration.linear.x -= 10.0;
        // }
    }
}
