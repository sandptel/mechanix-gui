use bevy::{ ecs::system::SystemId, prelude::* };
use bevy_smithay::{
    SmithayWindowType,
    prelude::{ layer_shell::LayerShellSettings, subsurface::Anchor },
};
use bevy_styled_widgets::prelude::{ ButtonVariant, StyledButton, StyledText };

use super::plugin::ScreenshotEvent;
use crate::styled_card::StyledCard;
use super::plugin::ScreenshotPlugin;

#[derive(Component)]
pub struct ScreenshotOverlayWindow;

#[derive(Component)]
pub struct ScreenshotOverlay;

pub struct ScreenshotUiPlugin;

impl Plugin for ScreenshotUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_screenshot_overlay).add_plugins(ScreenshotPlugin);
    }
}

fn trigger_screenshot_overlay(
    mut commands: Commands,
    mut screenshot_events: EventReader<ScreenshotEvent>,
    q_existing_overlay: Query<Entity, With<ScreenshotOverlayWindow>>
) {
    for _event in screenshot_events.read() {
        // Close any existing overlay first
        // for entity in q_existing_overlay.iter() {
        //     commands.entity(entity).despawn_recursive();
        // }

        spawn_screenshot_overlay(&mut commands);
    }
}

fn spawn_screenshot_overlay(commands: &mut Commands) {}

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
