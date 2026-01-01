use gpui::{Size, prelude::FluentBuilder, *};

use crate::ui::icon::{Icon, IconName};
pub mod icon;

const WING_WIDTH: f32 = 180.;
const WING_HEIGHT: f32 = 38.;
const CARD_WIDTH: f32 = 540.;
const CARD_HEIGHT: f32 = 620.;
const UPWARD_CANCEL_DISTANCE: f32 = 25.0;
const MAX_UPWARD_DRAG: f32 = 60.0;

const INIT_ANIMATE_HEIGHT: f32 = 0.0; // Start from top

pub struct PowerOptions {
    pub power_off: bool,

    // Drag state
    drag_offset: Option<f32>,
    drag_start_pos: f32,
    position_y: f32, // Current Y position of the swipe card

    // Animation state
    initial_height: f32, // Height of the amber card during initial animation
    is_initial_animation_done: bool,

    // Thresholds
    drag_threshold: f32,
    max_drag_distance: f32,

    // swipe upwards- go back
    drag_start_y: f32,
    upward_cancel_distance: f32,
    max_upward_drag: f32,
}

impl PowerOptions {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            power_off: false,
            drag_offset: None,
            drag_start_pos: 0.0,
            position_y: 0.0,
            initial_height: INIT_ANIMATE_HEIGHT,
            is_initial_animation_done: false,
            drag_threshold: 200.0,
            max_drag_distance: CARD_HEIGHT,
            drag_start_y: 0.0,
            upward_cancel_distance: UPWARD_CANCEL_DISTANCE,
            max_upward_drag: MAX_UPWARD_DRAG,
        };

        // Start initial reveal animation
        this.animate_initial_reveal(cx);
        this
    }

    fn handle_upward_swipe(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Dismiss the overlay when user intentionally drags upward to cancel.
        println!("Upward swipe detected, closing power options overlay.");
        window.remove_window();
        cx.notify();
    }

    fn animate_initial_reveal(&mut self, cx: &mut Context<Self>) {
        let start_height = 0.0; // Start from very top
        let target_height = CARD_HEIGHT / 2.0; // Go to half of card height (310px)
        let duration_ms = 1000.0; // Smooth animation duration
        let start_time = std::time::Instant::now();

        cx.spawn(
            async move |this: WeakEntity<PowerOptions>, cx: &mut AsyncApp| {
                loop {
                    let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                    if elapsed >= duration_ms {
                        this.update(cx, |this, cx| {
                            this.initial_height = target_height;
                            this.is_initial_animation_done = true;
                            cx.notify();
                        })
                        .ok();
                        break;
                    }

                    let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                    let ease = 1.0 - (1.0 - t).powi(3); // Cubic ease-out
                    let current_height = start_height + (target_height - start_height) * ease;

                    this.update(cx, |this, cx| {
                        this.initial_height = current_height;
                        cx.notify();
                    })
                    .ok();

                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(16))
                        .await;
                }
            },
        )
        .detach();
    }

    fn snap_to(&mut self, target: f32, cx: &mut Context<Self>) {
        let start = self.position_y;
        let change = target - start;
        let duration_ms = 250.0;
        let start_time = std::time::Instant::now();

        cx.spawn(
            async move |this: WeakEntity<PowerOptions>, cx: &mut AsyncApp| {
                loop {
                    let elapsed = start_time.elapsed().as_secs_f32() * 1000.0;

                    if elapsed >= duration_ms {
                        this.update(cx, |this, cx| {
                            this.position_y = target;

                            // Only trigger power off if the card is swiped ALL the way to bottom
                            // The card reaches bottom when: initial_height + position_y >= CARD_HEIGHT
                            let total_height = this.initial_height + target;
                            if total_height >= CARD_HEIGHT {
                                this.power_off = true;
                                println!("Power off triggered!");

                                // todo!("Implement actual power off logic here. To fix the Blank Screen at the end");
                            }

                            cx.notify();
                        })
                        .ok();
                        break;
                    }

                    let t = (elapsed / duration_ms).clamp(0.0, 1.0);
                    let ease = 1.0 - (1.0 - t).powi(3); // Cubic ease-out
                    let current = start + (change * ease);

                    this.update(cx, |this, cx| {
                        this.position_y = current;
                        cx.notify();
                    })
                    .ok();

                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(16))
                        .await;
                }
            },
        )
        .detach();
    }
}

impl Render for PowerOptions {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let size: Size<Pixels> = window.bounds().size;
        let max_drag = self.max_drag_distance;
        let threshold = self.drag_threshold;
        let initial_h = self.initial_height;
        let max_up_drag = self.max_upward_drag;
        let upward_cancel_distance = self.upward_cancel_distance;
        let _ = size; // keep viewport info accessible for future layout work

        // If powered off, show simple "Bye" message and stop interaction

        if self.power_off {
            return div()
                .flex()
                .flex_col()
                .w(px(CARD_WIDTH))
                .h(px(CARD_HEIGHT))
                .bg(rgb(0x000000))
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_xl()
                        .text_color(rgb(0xFFFFFF))
                        .font_weight(FontWeight::BOLD)
                        .child("Bye Comet!"),
                );
        }

        // Calculate dynamic height for the amber card
        let amber_card_height = if self.is_initial_animation_done {
            // After initial animation, expand based on drag
            self.initial_height + self.position_y
        } else {
            // During initial animation
            self.initial_height
        };

        // Calculate arrow height - shrinks as card approaches bottom
        let arrow_height = if self.is_initial_animation_done {
            let remaining = CARD_HEIGHT - amber_card_height;
            remaining.min(60.0).max(0.0)
        } else {
            0.0 // Hidden during initial animation
        };

        div()
            .flex()
            .flex_col()
            .relative()
            .w(px(CARD_WIDTH))
            .h(px(CARD_HEIGHT))
            .bg(rgb(0x1a1a1a))
            // Mouse move listener on parent container
            .on_mouse_move(cx.listener(move |this, event: &MouseMoveEvent, _, cx| {
                if let Some(offset) = this.drag_offset {
                    let new_y = event.position.y.to_f64() as f32 - offset;
                    // Respect both the configured drag limit and total card height
                    let clamped_max_drag = max_drag.max(initial_h);
                    let max_position = (clamped_max_drag - initial_h).min(CARD_HEIGHT - initial_h);
                    let min_position = -max_up_drag;
                    this.position_y = new_y.clamp(min_position, max_position);
                    cx.notify();
                }
            }))
            // Mouse up listener on parent container
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    if this.drag_offset.is_some() {
                        this.drag_offset = None;

                        let upward_distance = this.drag_start_y - this.position_y;
                        if upward_distance > upward_cancel_distance {
                            // Swiped upward significantly
                            this.handle_upward_swipe(window, cx);
                            return;
                        }

                        // After initial animation, card is at 310px (CARD_HEIGHT / 2)
                        // Remaining space from 310 to 620 is also 310px
                        // Use the configured drag threshold (capped by half remaining) to snap
                        let remaining_space = CARD_HEIGHT - initial_h; // 310px remaining
                        let half_remaining = remaining_space / 2.0; // 155px baseline threshold
                        let snap_threshold = threshold.min(half_remaining);

                        println!(
                            "position_y: {}, half_remaining: {}, remaining_space: {}",
                            this.position_y, half_remaining, remaining_space
                        );

                        // Determine snap target
                        let target = if this.position_y >= snap_threshold {
                            // Dragged more than half of remaining space - snap to bottom
                            remaining_space // This will make total height = 620
                        } else {
                            // Dragged less than half - snap back to initial position
                            0.0
                        };

                        this.snap_to(target, cx);
                        cx.notify();
                    }
                }),
            )
            .child(
                // Upper swipe area - THE DRAGGABLE CARD
                div()
                    .id("power-off-swipe-area")
                    .h(px(amber_card_height))
                    .w_full()
                    .bg(rgb(0x2d1f0f))
                    .rounded_b(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    // Only enable dragging after initial animation
                    .when(self.is_initial_animation_done, |this| {
                        this.on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, event: &MouseDownEvent, _, cx| {
                                cx.stop_propagation();

                                this.drag_start_y = this.position_y;
                                this.drag_start_pos = this.position_y;
                                this.drag_offset =
                                    Some(event.position.y.to_f64() as f32 - this.position_y);

                                cx.notify();
                            }),
                        )
                    })
                    //  .child(
                    //     div()
                    //         .flex()
                    //         .items_center()
                    //         .gap_3()
                    //         .child(
                    //             Icon::new(IconName::PowerOff)
                    //                 .text_color(rgb(0xC67600))
                    //                 .size((px(32.), px(32.))),
                    //         )
                    //         .child(
                    //             div()
                    //                 .text_xl()
                    //                 .text_color(rgb(0xd4a574))
                    //                 .child("Swipe to power off"),
                    //         ),
                    // ),
                    .when(!self.power_off, |this| {
                        this.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    Icon::new(IconName::PowerOff)
                                        .text_color(rgb(0xC67600))
                                        .size((px(32.), px(32.))),
                                )
                                .child(
                                    div()
                                        .text_xl()
                                        .text_color(rgb(0xd4a574))
                                        .child("Swipe to power off"),
                                ),
                        )
                    }),
            )
            .child(
                // Swipe indicator - shrinks and disappears as card approaches bottom
                div()
                    .h(px(arrow_height))
                    .w_full()
                    .bg(rgb(0x000000))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_center()
                    .when(arrow_height > 20.0, |div| {
                        div.child(
                            Icon::new(IconName::DownArrow)
                                .text_color(rgb(0xC67600))
                                .size((px(26.), px(26.))),
                        )
                    }),
            )
            .child(
                // Lower area - black area (flex to fill remaining space)
                div().flex_1().w_full().bg(rgb(0x000000)),
            )
    }
}
