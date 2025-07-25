use egui::{
    Align, Align2, Color32, DragValue, NumExt as _, Rect, ScrollArea, Sense, Slider, TextStyle,
    TextWrapMode, Ui, Vec2, Widget as _, pos2, scroll_area::ScrollBarVisibility,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
enum ScrollDemo {
    ScrollAppearance,
    ScrollTo,
    ManyLines,
    LargeCanvas,
    StickToEnd,
    Bidirectional,
}

impl Default for ScrollDemo {
    fn default() -> Self {
        Self::ScrollAppearance
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Default, PartialEq)]
pub struct Scrolling {
    appearance: ScrollAppearance,
    demo: ScrollDemo,
    scroll_to: ScrollTo,
    scroll_stick_to: ScrollStickTo,
}

impl crate::Demo for Scrolling {
    fn name(&self) -> &'static str {
        "↕ Scrolling"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .show(ctx, |ui| {
                use crate::View as _;
                self.ui(ui);
            });
    }
}

impl crate::View for Scrolling {
    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.demo, ScrollDemo::ScrollAppearance, "Appearance");
            ui.selectable_value(&mut self.demo, ScrollDemo::ScrollTo, "Scroll to");
            ui.selectable_value(
                &mut self.demo,
                ScrollDemo::ManyLines,
                "Scroll a lot of lines",
            );
            ui.selectable_value(
                &mut self.demo,
                ScrollDemo::LargeCanvas,
                "Scroll a large canvas",
            );
            ui.selectable_value(&mut self.demo, ScrollDemo::StickToEnd, "Stick to end");
            ui.selectable_value(&mut self.demo, ScrollDemo::Bidirectional, "Bidirectional");
        });
        ui.separator();
        match self.demo {
            ScrollDemo::ScrollAppearance => {
                self.appearance.ui(ui);
            }
            ScrollDemo::ScrollTo => {
                self.scroll_to.ui(ui);
            }
            ScrollDemo::ManyLines => {
                huge_content_lines(ui);
            }
            ScrollDemo::LargeCanvas => {
                huge_content_painter(ui);
            }
            ScrollDemo::StickToEnd => {
                self.scroll_stick_to.ui(ui);
            }
            ScrollDemo::Bidirectional => {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
                    for _ in 0..100 {
                        ui.label(crate::LOREM_IPSUM);
                    }
                });
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(PartialEq)]
struct ScrollAppearance {
    num_lorem_ipsums: usize,
    visibility: ScrollBarVisibility,
}

impl Default for ScrollAppearance {
    fn default() -> Self {
        Self {
            num_lorem_ipsums: 2,
            visibility: ScrollBarVisibility::default(),
        }
    }
}

impl ScrollAppearance {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let Self {
            num_lorem_ipsums,
            visibility,
        } = self;

        let mut scroll = ui.ctx().style().spacing.scroll;

        scroll.ui(ui);

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("ScrollBarVisibility:");
            for option in ScrollBarVisibility::ALL {
                ui.selectable_value(visibility, option, format!("{option:?}"));
            }
        });
        ui.weak("When to show scroll bars; resize the window to see the effect.");

        ui.add_space(8.0);

        ui.ctx().all_styles_mut(|s| s.spacing.scroll = scroll);

        ui.separator();

        ui.add(
            egui::Slider::new(num_lorem_ipsums, 1..=100)
                .text("Content length")
                .logarithmic(true),
        );

        ui.separator();

        ScrollArea::vertical()
            .auto_shrink(false)
            .scroll_bar_visibility(*visibility)
            .show(ui, |ui| {
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                    |ui| {
                        for _ in 0..*num_lorem_ipsums {
                            ui.label(crate::LOREM_IPSUM_LONG);
                        }
                    },
                );
            });
    }
}

fn huge_content_lines(ui: &mut egui::Ui) {
    ui.label(
        "A lot of rows, but only the visible ones are laid out, so performance is still good:",
    );
    ui.add_space(4.0);

    let text_style = TextStyle::Body;
    let row_height = ui.text_style_height(&text_style);
    let num_rows = 10_000;
    ScrollArea::vertical().auto_shrink(false).show_rows(
        ui,
        row_height,
        num_rows,
        |ui, row_range| {
            for row in row_range {
                let text = format!("This is row {}/{}", row + 1, num_rows);
                ui.label(text);
            }
        },
    );
}

fn huge_content_painter(ui: &mut egui::Ui) {
    // This is similar to the other demo, but is fully manual, for when you want to do custom painting.
    ui.label("A lot of rows, but only the visible ones are painted, so performance is still good:");
    ui.add_space(4.0);

    let font_id = TextStyle::Body.resolve(ui.style());
    let row_height = ui.fonts(|f| f.row_height(&font_id)) + ui.spacing().item_spacing.y;
    let num_rows = 10_000;

    ScrollArea::vertical()
        .auto_shrink(false)
        .show_viewport(ui, |ui, viewport| {
            ui.set_height(row_height * num_rows as f32);

            let first_item = (viewport.min.y / row_height).floor().at_least(0.0) as usize;
            let last_item = (viewport.max.y / row_height).ceil() as usize + 1;
            let last_item = last_item.at_most(num_rows);

            let mut used_rect = Rect::NOTHING;

            for i in first_item..last_item {
                let indentation = (i % 100) as f32;
                let x = ui.min_rect().left() + indentation;
                let y = ui.min_rect().top() + i as f32 * row_height;
                let text = format!(
                    "This is row {}/{}, indented by {} pixels",
                    i + 1,
                    num_rows,
                    indentation
                );
                let text_rect = ui.painter().text(
                    pos2(x, y),
                    Align2::LEFT_TOP,
                    text,
                    font_id.clone(),
                    ui.visuals().text_color(),
                );
                used_rect |= text_rect;
            }

            ui.allocate_rect(used_rect, Sense::hover()); // make sure it is visible!
        });
}

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(PartialEq)]
struct ScrollTo {
    track_item: usize,
    tack_item_align: Option<Align>,
    offset: f32,
    delta: f32,
}

impl Default for ScrollTo {
    fn default() -> Self {
        Self {
            track_item: 25,
            tack_item_align: Some(Align::Center),
            offset: 0.0,
            delta: 64.0,
        }
    }
}

impl crate::View for ScrollTo {
    fn ui(&mut self, ui: &mut Ui) {
        ui.label("This shows how you can scroll to a specific item or pixel offset");

        let num_items = 500;

        let mut track_item = false;
        let mut go_to_scroll_offset = false;
        let mut scroll_top = false;
        let mut scroll_bottom = false;
        let mut scroll_delta = None;

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific item index:");
            track_item |= ui
                .add(Slider::new(&mut self.track_item, 1..=num_items).text("Track Item"))
                .dragged();
        });

        ui.horizontal(|ui| {
            ui.label("Item align:");
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Some(Align::Min), "Top")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Some(Align::Center), "Center")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, Some(Align::Max), "Bottom")
                .clicked();
            track_item |= ui
                .radio_value(&mut self.tack_item_align, None, "None (Bring into view)")
                .clicked();
        });

        ui.horizontal(|ui| {
            ui.label("Scroll to a specific offset:");
            go_to_scroll_offset |= ui
                .add(DragValue::new(&mut self.offset).speed(1.0).suffix("px"))
                .dragged();
        });

        ui.horizontal(|ui| {
            scroll_top |= ui.button("Scroll to top").clicked();
            scroll_bottom |= ui.button("Scroll to bottom").clicked();
        });

        ui.horizontal(|ui| {
            ui.label("Scroll by");
            DragValue::new(&mut self.delta)
                .speed(1.0)
                .suffix("px")
                .ui(ui);
            if ui.button("⬇").clicked() {
                scroll_delta = Some(self.delta * Vec2::UP); // scroll down (move contents up)
            }
            if ui.button("⬆").clicked() {
                scroll_delta = Some(self.delta * Vec2::DOWN); // scroll up (move contents down)
            }
        });

        let mut scroll_area = ScrollArea::vertical().max_height(200.0).auto_shrink(false);
        if go_to_scroll_offset {
            scroll_area = scroll_area.vertical_scroll_offset(self.offset);
        }

        ui.separator();
        let (current_scroll, max_scroll) = scroll_area
            .show(ui, |ui| {
                if scroll_top {
                    ui.scroll_to_cursor(Some(Align::TOP));
                }
                if let Some(scroll_delta) = scroll_delta {
                    ui.scroll_with_delta(scroll_delta);
                }

                ui.vertical(|ui| {
                    for item in 1..=num_items {
                        if track_item && item == self.track_item {
                            let response =
                                ui.colored_label(Color32::YELLOW, format!("This is item {item}"));
                            response.scroll_to_me(self.tack_item_align);
                        } else {
                            ui.label(format!("This is item {item}"));
                        }
                    }
                });

                if scroll_bottom {
                    ui.scroll_to_cursor(Some(Align::BOTTOM));
                }

                let margin = ui.visuals().clip_rect_margin;

                let current_scroll = ui.clip_rect().top() - ui.min_rect().top() + margin;
                let max_scroll = ui.min_rect().height() - ui.clip_rect().height() + 2.0 * margin;
                (current_scroll, max_scroll)
            })
            .inner;
        ui.separator();

        ui.label(format!(
            "Scroll offset: {current_scroll:.0}/{max_scroll:.0} px"
        ));

        ui.separator();
        ui.vertical_centered(|ui| {
            egui::reset_button(ui, self, "Reset");
            ui.add(crate::egui_github_link_file!());
        });
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Default, PartialEq)]
struct ScrollStickTo {
    n_items: usize,
}

impl crate::View for ScrollStickTo {
    fn ui(&mut self, ui: &mut Ui) {
        ui.label("Rows enter from the bottom, we want the scroll handle to start and stay at bottom unless moved");

        ui.add_space(4.0);

        let text_style = TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        ScrollArea::vertical().stick_to_bottom(true).show_rows(
            ui,
            row_height,
            self.n_items,
            |ui, row_range| {
                for row in row_range {
                    let text = format!("This is row {}", row + 1);
                    ui.label(text);
                }
            },
        );

        self.n_items += 1;
        ui.ctx().request_repaint();
    }
}
