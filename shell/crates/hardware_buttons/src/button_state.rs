use gpui::*;
use mechanix_hw_buttons::{Key, KeyEvent};

#[derive(Debug, Default)]
pub struct ButtonState {
    last_home: Option<KeyEvent>,
    last_power: Option<KeyEvent>,
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

    pub fn handle_event(&mut self, event: KeyEvent) -> bool {
        match event {
            KeyEvent::Pressed(Key::Home) => {
                self.last_home = Some(event);
                println!("[hardware-buttons] Home button pressed: {:?}", event);
                true
            }
            KeyEvent::Pressed(Key::Power) => {
                self.last_power = Some(event);
                println!("[hardware-buttons] Power button pressed: {:?}", event);
                true
            }
            _ => false,
        }
    }
}