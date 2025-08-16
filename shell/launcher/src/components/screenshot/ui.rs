use bevy::{ ecs::system::SystemId, prelude::* };
use bevy_smithay::{
    SmithayWindowType,
    prelude::{ layer_shell::LayerShellSettings, subsurface::Anchor },
};
use bevy_styled_widgets::prelude::{ ButtonVariant, StyledButton, StyledText };
use crate::launcher::spawn_camera;
use super::plugin::ScreenshotEvent;
use crate::styled_card::StyledCard;
use super::plugin::ScreenshotPlugin;

#[derive(Component)]
pub struct ScreenshotWindow;

#[derive(Component)]
pub struct ScreenshotOverlay;

pub struct ScreenshotUiPlugin;

impl Plugin for ScreenshotUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_screenshot_window)
        .add_systems(Update, despawn_screenshot_window)
        .add_plugins(ScreenshotPlugin);

    }
}

#[derive(Resource)]
pub struct ScreenshotWindowSurface(pub Entity);

fn trigger_screenshot_window(
    mut commands: Commands,
    mut screenshot_events: EventReader<ScreenshotEvent>,
    q_existing_overlay: Query<Entity, With<ScreenshotWindow>>
) {
    for _event in screenshot_events.read() {
        // Close any existing overlay first
        // for entity in q_existing_overlay.iter() {
        //     commands.entity(entity).despawn_recursive();
        // }

        let camera_entity = spawn_camera(
            &mut commands,
            540,
            531,
            "Screenshot Overlay".to_string(),
            LayerShellSettings {
                layer: bevy_smithay::prelude::subsurface::Layer::Overlay,
                anchor: Anchor::LEFT | Anchor::RIGHT | Anchor::TOP,
                exclusive_zone: 0,
                ..default()
            },
            ScreenshotOverlay
        );

        // Spawn transparent UI root
        let screenshot_window_surface = commands
            .spawn((
                UiTargetCamera(camera_entity),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ScreenshotWindow,
                BackgroundColor(Color::BLACK),
            ))
            .id();

        commands.insert_resource(ScreenshotWindowSurface(screenshot_window_surface));
    }
}

// fn

fn despawn_screenshot_window(
    mut commands: Commands,
    q_existing_window: Query<Entity, With<ScreenshotOverlay>>,
    keys: Res<ButtonInput<KeyCode>>,
    
) {
    if keys.just_pressed(KeyCode::KeyA) {
        println!("Despawning screenshot overlay window");
        for entity in q_existing_window.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_screenshot_overlay_window(commands: &mut Commands) {}

// fn spawn_screenshot_camera(
//     commands: &mut Commands,
//     width: u32,
//     height: u32,
//     title: String,
//     mut settings: LayerShellSettings,
//     marker: impl Bundle
// ) -> Entity {
//     settings.size = (width, height);
// }

// fn screenshot_options_ui(
//     on_fullscreen: SystemId,
//     on_region: SystemId,
//     on_cancel: SystemId
// ) -> impl Bundle {
//     children![]
// }
