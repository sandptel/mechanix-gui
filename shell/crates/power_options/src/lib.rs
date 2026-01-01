use commons::assets::Assets;
use gpui::*;

mod ui;

use crate::ui::PowerOptions;

pub mod prelude {
    // pub use crate::events::AppEvents;
    pub use crate::run_app;
}

pub fn run_app(cx: &mut App) {
    let application = Application::new().with_assets(Assets {});
    application.run(|cx| {
        let window_bounds =
            WindowBounds::Windowed(Bounds::centered(None, size(px(540.0), px(620.0)), cx));

        cx.open_window(
            WindowOptions {
                window_bounds: Some(window_bounds),
                // kind: WindowKind::LayerShell(LayerShellOptions {
                //     namespace: "mechanix.settings_drawer".to_string(),
                //     layer: Layer::Top,
                //     anchor: Anchor::LEFT | Anchor::TOP | Anchor::RIGHT | Anchor::BOTTOM,
                //     keyboard_interactivity: KeyboardInteractivity::None,
                //     margin: None,
                //     exclusive_zone: None,
                //     ..Default::default()
                // }),
                ..Default::default()
            },
            |_window, cx| cx.new(|cx| PowerOptions::new(cx)),
        )
        .unwrap();
        cx.activate(true);
    });
}
