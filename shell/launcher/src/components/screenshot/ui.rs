use bevy::prelude::*;
use bevy_smithay::{
    prelude::{ layer_shell::LayerShellSettings, subsurface::Anchor },
};
use crate::launcher::spawn_camera;
use super::plugin::CurrentScreenshotImage;
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

#[derive(Component)]
pub struct SaveDialog;

#[derive(Component)]
pub struct DialogOverlay;

#[derive(Component)]
pub struct DialogTimer(pub Timer);

#[derive(Component)]
pub struct YesButton;

#[derive(Component)]
pub struct NoButton;

pub struct ScreenshotUiPlugin;

impl Plugin for ScreenshotUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_screenshot_window.run_if(resource_exists_and_changed::<CurrentScreenshotImage>))
            .add_systems(
                Update,
                spawn_screenshot_ui.run_if(resource_exists::<ScreenshotWindowSurface>)
            )
            .add_systems(Update, despawn_screenshot_window)
            .add_systems(Update, (
                save_button_interaction,
                delete_button_interaction,
                copy_button_interaction,
                auto_hide_dialogs,
                yes_button_interaction,
                no_button_interaction,
            ))
            .add_plugins(ScreenshotPlugin);
    }
}


#[derive(Resource)]
pub struct ScreenshotWindowSurface(pub Entity);

fn trigger_screenshot_window(
    mut commands: Commands,
    _current_screenshot: Res<CurrentScreenshotImage>,
    existing_overlays: Query<Entity, With<ScreenshotOverlay>>,
    _existing_window: Option<Res<ScreenshotWindowSurface>>,
) {
    // Check if we already have an overlay, if so, stack on top
    if !existing_overlays.is_empty() {
        println!("Screenshot overlay already exists, creating new overlay on top");
    }

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
    current_screenshot: Option<Res<CurrentScreenshotImage>>,
    asset_server: Res<AssetServer>
) {
    // Only spawn UI when we have screenshot data
    let current_screenshot = current_screenshot.unwrap();
    let screenshot_image = current_screenshot.image.clone();

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
                            width: Val::Auto,
                            height: Val::Auto,
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
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SaveButton>)>,
    current_screenshot: Option<Res<CurrentScreenshotImage>>,
    mut commands: Commands,
    screenshot_window: Option<Res<ScreenshotWindowSurface>>,
    existing_dialog: Query<Entity, With<SaveDialog>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Save button was pressed");
                
                if let Some(current_screenshot) = &current_screenshot {
                    // Remove any existing dialog first
                    for entity in existing_dialog.iter() {
                        commands.entity(entity).despawn();
                    }
                    
                    // Spawn save confirmation dialog
                    if let Some(window) = &screenshot_window {
                        spawn_save_confirmation_dialog(&mut commands, window.0, current_screenshot.image.clone());
                    }
                } else {
                    eprintln!("No screenshot data available");
                }
            }
            _ => {}
        }
    }
}

fn delete_button_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<DeleteButton>)>,
    current_screenshot: Option<Res<CurrentScreenshotImage>>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Delete button was pressed");
                if let Some(current_screenshot) = &current_screenshot {
                    // Try to delete the file if it exists
                    if current_screenshot.output_path.exists() {
                        match std::fs::remove_file(&current_screenshot.output_path) {
                            Ok(()) => println!("Screenshot file deleted successfully"),
                            Err(e) => eprintln!("Failed to delete screenshot file: {}", e),
                        }
                    }
                    // Remove the resource
                    commands.remove_resource::<CurrentScreenshotImage>();
                }
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

fn yes_button_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<YesButton>)>,
    current_screenshot: Option<Res<CurrentScreenshotImage>>,
    images: Res<Assets<Image>>,
    mut commands: Commands,
    screenshot_window: Option<Res<ScreenshotWindowSurface>>,
    existing_dialog: Query<Entity, With<SaveDialog>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Yes button was pressed - saving screenshot");
                
                if let Some(current_screenshot) = &current_screenshot {
                    if let Some(image) = images.get(&current_screenshot.image) {
                        // Remove the confirmation dialog first
                        for entity in existing_dialog.iter() {
                            commands.entity(entity).despawn();
                        }
                        
                        // Try to save the image
                        match super::plugin::save_image_as_png(image, &current_screenshot.output_path) {
                            Ok(()) => {
                                println!("Screenshot saved successfully");
                                
                                // Spawn save success dialog
                                if let Some(window) = &screenshot_window {
                                    spawn_save_success_dialog(&mut commands, window.0);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to save screenshot: {}", e);
                                
                                // Spawn error dialog
                                if let Some(window) = &screenshot_window {
                                    spawn_error_dialog(&mut commands, window.0, &format!("Failed to save: {}", e));
                                }
                            }
                        }
                    } else {
                        eprintln!("Screenshot image not found in assets");
                    }
                } else {
                    eprintln!("No screenshot data available");
                }
            }
            _ => {}
        }
    }
}

fn no_button_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<NoButton>)>,
    mut commands: Commands,
    existing_dialog: Query<Entity, With<SaveDialog>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("No button was pressed - canceling save");
                
                // Remove the confirmation dialog
                for entity in existing_dialog.iter() {
                    commands.entity(entity).despawn();
                }
            }
            _ => {}
        }
    }
}

fn auto_hide_dialogs(
    mut commands: Commands,
    mut dialog_query: Query<(Entity, &mut DialogTimer), With<SaveDialog>>,
    time: Res<Time>,
) {
    for (entity, mut timer) in dialog_query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_save_confirmation_dialog(commands: &mut Commands, parent: Entity, screenshot_image: Handle<Image>) {
    let dialog = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(400.0),
            height: Val::Px(500.0),
            margin: UiRect {
                left: Val::Px(-200.0), // Half of width for centering
                top: Val::Px(-250.0),   // Half of height for centering
                ..default()
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.95)),
        BorderColor(Color::srgba(0.4, 0.4, 0.4, 1.0)),
        BorderRadius::all(Val::Px(12.0)),
        SaveDialog,
        DialogOverlay,
    )).with_children(|dialog| {
        // Screenshot preview container
        dialog.spawn((
            Node {
                width: Val::Px(320.0),
                height: Val::Px(240.0),
                margin: UiRect::bottom(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 1.0)),
            BorderColor(Color::srgba(0.3, 0.3, 0.3, 1.0)),
            BorderRadius::all(Val::Px(8.0)),
        )).with_children(|preview_container| {
            // Screenshot image
            preview_container.spawn((
                ImageNode::new(screenshot_image),
                Node {
                    width: Val::Percent(90.0),
                    height: Val::Percent(90.0),
                    ..default()
                },
            ));
        });
        
        // Save confirmation message
        dialog.spawn((
            Text::new("Save screenshot to Screenshots folder?"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
        ));
        
        // Button container
        dialog.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                ..default()
            },
        )).with_children(|buttons| {
            // Yes button
            buttons.spawn((
                Button,
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.7, 0.2, 1.0)),
                BorderColor(Color::srgba(0.3, 0.8, 0.3, 1.0)),
                BorderRadius::all(Val::Px(12.0)),
                YesButton,
            )).with_children(|button| {
                button.spawn((
                    Text::new("Yes"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
            
            // No button
            buttons.spawn((
                Button,
                Node {
                    width: Val::Px(80.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.7, 0.2, 0.2, 1.0)),
                BorderColor(Color::srgba(0.8, 0.3, 0.3, 1.0)),
                BorderRadius::all(Val::Px(12.0)),
                NoButton,
            )).with_children(|button| {
                button.spawn((
                    Text::new("No"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
    }).id();
    
    commands.entity(parent).add_child(dialog);
}

fn spawn_save_success_dialog(commands: &mut Commands, parent: Entity) {
    let dialog = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(300.0),
            height: Val::Px(150.0),
            margin: UiRect {
                left: Val::Px(-150.0), // Half of width for centering
                top: Val::Px(-75.0),   // Half of height for centering
                ..default()
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.7, 0.2, 0.9)),
        BorderColor(Color::srgba(0.3, 0.8, 0.3, 1.0)),
        BorderRadius::all(Val::Px(12.0)),
        SaveDialog,
        DialogOverlay,
        DialogTimer(Timer::from_seconds(3.0, TimerMode::Once)), // Auto-hide after 3 seconds
    )).with_children(|parent| {
        // Success message
        parent.spawn((
            Text::new("Screenshot saved successfully!"),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    }).id();
    
    commands.entity(parent).add_child(dialog);
}

fn spawn_error_dialog(commands: &mut Commands, parent: Entity, error_message: &str) {
    let dialog = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(350.0),
            height: Val::Px(180.0),
            margin: UiRect {
                left: Val::Px(-175.0), // Half of width for centering
                top: Val::Px(-90.0),   // Half of height for centering
                ..default()
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.8, 0.2, 0.2, 0.9)),
        BorderColor(Color::srgba(0.9, 0.3, 0.3, 1.0)),
        BorderRadius::all(Val::Px(12.0)),
        SaveDialog,
        DialogOverlay,
        DialogTimer(Timer::from_seconds(4.0, TimerMode::Once)), // Auto-hide after 4 seconds for errors
    )).with_children(|parent| {
        // Error message
        parent.spawn((
            Text::new(error_message),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));
    }).id();
    
    commands.entity(parent).add_child(dialog);
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
