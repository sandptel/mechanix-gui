use gpui::*;
use mechanix_hw_buttons::{Key, KeyEvent};
use power_options::run_app as run_power_overlay;

#[derive(Debug, Default)]
pub struct ButtonState {
    last_home: Option<KeyEvent>,
    last_power: Option<KeyEvent>,
    power_overlay_visible: bool,
}

impl Global for ButtonState {}

impl ButtonState {
    pub fn init(cx: &mut App) {
        if !cx.has_global::<ButtonState>() {
            cx.set_global(ButtonState::default());
        }
    }

    pub fn global_mut(cx: &mut App) -> &mut ButtonState {
        cx.global_mut::<ButtonState>()
    }

    pub fn handle_event(cx: &mut App, event: KeyEvent) -> bool {
        let mut launch_overlay = false;

        let handled = {
            let state = ButtonState::global_mut(cx);
            match event {
                KeyEvent::Pressed(Key::Home) => {
                    state.last_home = Some(event);
                    println!("[hardware-buttons] Home button short press: {:?}", event);
                    // todo!(): Handle short home-button action (maybe: return to launcher).
                    true
                }
                KeyEvent::Pressing(Key::Home) => {
                    println!("[hardware-buttons] Home button long press detected: {:?}", event);
                    // todo!(): Handle long home-button action (maybe: open multitasking view).
                    true
                }
                KeyEvent::Released(Key::Home) => {
                    state.last_home = Some(event);
                    println!("[hardware-buttons] Home button released: {:?}", event);
                    true
                }
                KeyEvent::Pressed(Key::Power) => {
                    state.last_power = Some(event);
                    println!("[hardware-buttons] Power button short press: {:?}", event);
                    if !state.power_overlay_visible {
                        // todo!(): Launch lock screen on short power-button presses.
                    }
                    true
                }
                KeyEvent::Pressing(Key::Power) => {
                    if !state.power_overlay_visible {
                        println!(
                            "[hardware-buttons] Power button long press detected, showing power overlay"
                        );
                        state.power_overlay_visible = true;
                        launch_overlay = true;
                    }
                    true
                }
                KeyEvent::Released(Key::Power) => {
                    state.last_power = Some(event);
                    // # TO_CONFIGRM : This was the logic to remove the overlay on release, but we want to keep it until user dismisses it
                    // state.power_overlay_visible = false;
                    println!("[hardware-buttons] Power button released: {:?}", event);
                    true
                }
                _ => false,
            }
        };

        if launch_overlay {
            run_power_overlay(cx);
        }

        handled
    }
}