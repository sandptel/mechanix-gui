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

#[derive(Component)]
pub struct SaveButton;

#[derive(Component)]
pub struct DeleteButton;

#[derive(Component)]
pub struct CopyButton;

pub struct ScreenshotUiPlugin;

impl Plugin for ScreenshotUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_screenshot_window)
            .add_systems(
                Update,
                spawn_screenshot_ui.run_if(resource_exists_and_changed::<ScreenshotWindowSurface>)
            )
            .add_systems(Update, despawn_screenshot_window)
            .add_systems(Update, (
                save_button_interaction,
                delete_button_interaction,
                copy_button_interaction,
            ))
            .add_plugins(ScreenshotPlugin);
    }
}

use std::path::PathBuf;

#[derive(Resource)]
pub struct ScreenshotWindowSurface(pub Entity, pub Handle<Image>);

fn trigger_screenshot_window(
    mut commands: Commands,
    mut screenshot_events: EventReader<ScreenshotEvent>,
    asset_server: Res<AssetServer>
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
        let screenshot_asset = asset_server.load("icons/camera.png");
        commands.insert_resource(
            ScreenshotWindowSurface(screenshot_window_surface, screenshot_asset)
        );
    }
}

// fn

fn despawn_screenshot_window(
    mut commands: Commands,
    q_existing_window: Query<Entity, With<ScreenshotOverlay>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    if keys.just_pressed(KeyCode::KeyA) {
        println!("Despawning screenshot overlay window");
        for entity in q_existing_window.iter() {
            commands.entity(entity).despawn();
        }
    }
}

// fn spawn_image_node(image: Handle<Image>) -> impl Bundle {
//     (
//         Node {
//             width: Val::Percent(100.0),
//             height: Val::Auto,
//             align_items: AlignItems::Center,
//             justify_content: JustifyContent::Center,
//             ..default()
//         },
//         children![(
//             ImageNode::new(image),
//             Node {
//                 width: Val::Percent(75.0),
//                 height: Val::Percent(75.0),
//                 ..Default::default()
//             },
//         )],
//     )
// }

fn spawn_screenshot_ui(
    mut commands: Commands,
    screenshot_window: Res<ScreenshotWindowSurface>,
    asset_server: Res<AssetServer>
) {
    let screenshot_image = screenshot_window.1.clone();

    // Load button icons
    let save_icon = asset_server.load("icons/screenshot_save.png"); // Using camera icon as placeholder for save
    let delete_icon = asset_server.load("icons/screenshot_delete.png"); // Using terminal icon as placeholder for delete
    let copy_icon = asset_server.load("icons/screenshot_copy.png"); // Using calculator icon as placeholder for copy

    commands.entity(screenshot_window.0).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Percent(5.0)), // 5% padding around the entire content
                ..default()
            },
            children![
                // Screenshot image container
                (
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(95.0),
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Percent(5.0)), // Small margin between image and buttons
                        ..default()
                    },
                    children![(
                        ImageNode::new(screenshot_image),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                    )],
                ),
                // Button container box
                (
                    Node {
                        width: Val::Px(540.0),
                        height: Val::Px(106.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgba(0.183, 0.183, 0.183, 1.0)),
                    children![
                        // Save Button
                        (
                            Button::default(),
                            Node {
                                width: Val::Px(80.0),
                                height: Val::Px(80.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            SaveButton,
                            children![(
                                ImageNode::new(save_icon),
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    ..default()
                                },
                            )],
                        ),
                        // Delete Button
                        (
                            Button::default(),
                            Node {
                                width: Val::Px(80.0),
                                height: Val::Px(80.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            DeleteButton,
                            children![(
                                ImageNode::new(delete_icon),
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    ..default()
                                },
                            )],
                        ),
                        // Copy Button
                        (
                            Button::default(),
                            Node {
                                width: Val::Px(80.0),
                                height: Val::Px(80.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            CopyButton,
                            children![(
                                ImageNode::new(copy_icon),
                                Node {
                                    width: Val::Px(40.0),
                                    height: Val::Px(40.0),
                                    ..default()
                                },
                            )],
                        )
                    ],
                )
            ],
        ));
    });
}

fn save_button_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SaveButton>)>
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Save button was pressed");
            }
            _ => {}
        }
    }
}

fn delete_button_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<DeleteButton>)>
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Delete button was pressed");
            }
            _ => {}
        }
    }
}

fn copy_button_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<CopyButton>)>
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Copy button was pressed");
            }
            _ => {}
        }
    }
}

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
