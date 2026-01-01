use gpui::layer_shell::{Anchor, KeyboardInteractivity, Layer, LayerShellOptions};
use gpui::*;

mod ui;

use crate::ui::PowerOptions;

pub mod prelude {
    // pub use crate::events::AppEvents;
    pub use crate::run_app;
}

const WINDOW_NAMESPACE: &str = "mechanix.power_options";
const WINDOW_WIDTH: f32 = 540.0;
const WINDOW_HEIGHT: f32 = 620.0;

pub fn run_app(cx: &mut App) {
    let window_bounds =
        WindowBounds::Windowed(Bounds::centered(None, size(px(WINDOW_WIDTH), px(WINDOW_HEIGHT)), cx));

    cx.open_window(
        WindowOptions {
            window_bounds: Some(window_bounds),
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace: WINDOW_NAMESPACE.into(),
                layer: Layer::Overlay,
                anchor: Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
                keyboard_interactivity: KeyboardInteractivity::None,
                exclusive_zone: Some(px(0.0)),
                ..Default::default()
            }),
            ..Default::default()
        },
        |_window, cx| cx.new(|cx| PowerOptions::new(cx)),
    )
    .expect("failed to open power options overlay");
    cx.activate(true);
}
