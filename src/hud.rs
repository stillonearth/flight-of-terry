use bevy::prelude::*;

#[derive(Component)]
pub struct TextVelocity;

#[derive(Component)]
pub struct TextRoll;

#[derive(Component)]
pub struct TextPitch;

#[derive(Component)]
pub struct TextYaw;

pub fn draw_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(Camera2dBundle::default());
    // Text with one section
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "VELOCITY ",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Roboto-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(TextVelocity);

    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    " ROLL ",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Roboto-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(TextRoll);

    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    " PITCH ",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Roboto-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(TextPitch);

    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    " YAW ",
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/Roboto-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(TextYaw);
}
