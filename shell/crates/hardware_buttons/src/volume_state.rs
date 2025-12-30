use gpui::*;
use mechanix_hw_buttons::{Key, KeyEvent};

use crate::slider::{SliderEvent, SliderState};

pub const DEFAULT_LEVEL: f32 = 35.0;
pub const MIN_LEVEL: f32 = 0.0;
pub const MAX_LEVEL: f32 = 100.0;

#[derive(Debug, Clone, Copy)]
pub struct VolumeConfig {
    pub step: f32,
}

impl Default for VolumeConfig {
    fn default() -> Self {
        Self { step: 5.0 }
    }
}

#[derive(Debug)]
pub struct VolumeState {
    slider: Option<Entity<SliderState>>,
    value: f32,
    step: f32,
}

impl Global for VolumeState {}

impl VolumeState {
    pub fn init(cx: &mut App) {
        Self::init_with_config(cx, VolumeConfig::default());
    }

    pub fn init_with_config(cx: &mut App, config: VolumeConfig) {
        if cx.has_global::<VolumeState>() {
            return;
        }

        let step = if config.step.abs() < f32::EPSILON {
            VolumeConfig::default().step
        } else {
            config.step.abs()
        };

        cx.set_global(VolumeState {
            slider: None,
            value: DEFAULT_LEVEL,
            step,
        });
    }

    pub fn global(cx: &App) -> &VolumeState {
        cx.global::<VolumeState>()
    }

    pub fn global_mut(cx: &mut App) -> &mut VolumeState {
        cx.global_mut::<VolumeState>()
    }

    pub fn current_value(&self) -> f32 {
        self.value
    }

    pub fn register_slider(
        &mut self,
        slider: &Entity<SliderState>,
    ) -> Option<(Entity<SliderState>, f32)> {
        self.slider = Some(slider.clone());
        self.slider.clone().map(|entity| (entity, self.value))
    }

    pub fn apply_key_event(
        &mut self,
        event: KeyEvent,
    ) -> Option<(Entity<SliderState>, f32)> {
        let delta = match event {
            KeyEvent::Pressed(Key::VolumeUp)
            | KeyEvent::Pressing(Key::VolumeUp)
            | KeyEvent::Unknown(Key::VolumeUp) => self.step,
            KeyEvent::Pressed(Key::VolumeDown)
            | KeyEvent::Pressing(Key::VolumeDown)
            | KeyEvent::Unknown(Key::VolumeDown) => -self.step,
            _ => return None,
        };

        self.value = (self.value + delta).clamp(MIN_LEVEL, MAX_LEVEL);
        self.slider.clone().map(|entity| (entity, self.value))
    }

}

pub fn apply_value_to_slider(
    slider: &Entity<SliderState>,
    value: f32,
    cx: &mut App,
) {
    slider.update(cx, |state, cx| {
        let clamped = value.clamp(state.min, state.max);
        if (state.value - clamped).abs() < f32::EPSILON {
            return;
        }
        state.value = clamped;
        cx.emit(SliderEvent::Change(state.value));
        cx.notify();
    });
}

pub fn sync_slider_value(
    slider: &Entity<SliderState>,
    value: f32,
    cx: &mut App,
) {
    slider.update(cx, |state, _| {
        state.value = value.clamp(state.min, state.max);
    });
}
