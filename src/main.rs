mod aerodynamics;
mod hud;
mod player;

use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::*};
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_terrain::prelude::*;

use crate::aerodynamics::*;
use crate::hud::*;
use crate::player::*;

// Materials

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "003e1d5d-241c-45a6-8c25-731dee22d820"]
pub struct TerrainMaterial {}

impl Material for TerrainMaterial {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TerrainPipelineConfig {
            attachment_count: 2, // has to match the attachments of the terrain
        })
        // Terrain Plugin
        .add_plugin(TerrainPlugin)
        .add_plugin(TerrainDebugPlugin) // enable debug settings and controls
        .add_plugin(TerrainMaterialPlugin::<TerrainMaterial>::default())
        // Atmosphere Plugin
        .add_plugin(AtmospherePlugin)
        // Physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        // .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
        // Startup Systems
        .add_startup_system(setup)
        .add_startup_system(draw_hud)
        // Gameplay systems
        .add_system(keyboard_control)
        .add_system(aerodynamical_forces.after(keyboard_control))
        .run();
}

// Level

const TERRAIN_SIZE: u32 = 1024;
const LOD_COUNT: u32 = 5;
const CHUNK_SIZE: u32 = 128;
const HEIGHT: f32 = 200.0;
const NODE_ATLAS_SIZE: u32 = 300;
const PATH: &str = "terrain";

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<TerrainMaterial>>,
    mut quadtrees: ResMut<TerrainViewComponents<Quadtree>>,
    mut view_configs: ResMut<TerrainViewComponents<TerrainViewConfig>>,
) {
    let mut preprocessor = Preprocessor::default();
    let mut from_disk_loader = AttachmentFromDiskLoader::default();

    // Configure all the important properties of the terrain, as well as its attachments.
    let mut config = TerrainConfig::new(
        TERRAIN_SIZE,
        CHUNK_SIZE,
        LOD_COUNT,
        HEIGHT,
        NODE_ATLAS_SIZE,
        PATH.to_string(),
    );

    config.add_base_attachment(
        &mut preprocessor,
        &mut from_disk_loader,
        CHUNK_SIZE,
        TileConfig {
            path: "assets/terrain/source/height".to_string(),
            lod: 0,
            offset: Default::default(),
            size: TERRAIN_SIZE,
        },
    );

    // Create the terrain.
    let terrain = commands
        .spawn_bundle(TerrainBundle::new(config.clone()))
        .insert(from_disk_loader)
        .insert(materials.add(TerrainMaterial {}))
        .id();

    // Configure the quality settings of the terrain view. Adapt the settings to your liking.
    let view_config = TerrainViewConfig::new(&config, 10, 5.0, 3.0, 10.0, 0.2, 0.2, 0.2);

    // Create plane
    let player_bundle = new_player_bundle(Vec3::new(250.0, 500.0, 250.0));
    let view = commands
        .spawn_bundle(player_bundle)
        .insert(GravityScale(0.0))
        .id();

    // Store the quadtree and the view config for the terrain and view.
    // This will hopefully be way nicer once the ECS can handle relations.
    let quadtree = Quadtree::from_configs(&config, &view_config);
    view_configs.insert((terrain, view), view_config);
    quadtrees.insert((terrain, view), quadtree);

    // Create a sunlight for the physical based lighting.
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 1.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // Preprocesses the terrain data.
    // Todo: Should be commented out after the first run.
    preprocessor.preprocess(&config);
}
