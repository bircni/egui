use crate::{
    widgets, Align, Color32, CornerRadius, FontSelection, Image, NumExt as _, Rect, Response,
    Sense, Stroke, TextStyle, TextWrapMode, Ui, Vec2, Widget, WidgetInfo, WidgetText, WidgetType,
};

/// Clickable button with text.
///
/// See also [`Ui::button`].
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// # fn do_stuff() {}
///
/// if ui.add(egui::Button::new("Click me")).clicked() {
///     do_stuff();
/// }
///
/// // A greyed-out and non-interactive button:
/// if ui.add_enabled(false, egui::Button::new("Can't click this")).clicked() {
///     unreachable!();
/// }
/// # });
/// ```
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
pub struct Button<'a> {
    image: Option<Image<'a>>,
    text: Option<WidgetText>,
    right_text: WidgetText,
    wrap_mode: Option<TextWrapMode>,

    /// None means default for interact
    fill: Option<Color32>,
    stroke: Option<Stroke>,
    sense: Sense,
    small: bool,
    frame: Option<bool>,
    min_size: Vec2,
    corner_radius: Option<CornerRadius>,
    selected: bool,
    image_tint_follows_text_color: bool,
}

impl<'a> Button<'a> {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self::opt_image_and_text(None, Some(text.into()))
    }

    /// Creates a button with an image. The size of the image as displayed is defined by the provided size.
    pub fn image(image: impl Into<Image<'a>>) -> Self {
        Self::opt_image_and_text(Some(image.into()), None)
    }

    /// Creates a button with an image to the left of the text. The size of the image as displayed is defined by the provided size.
    pub fn image_and_text(image: impl Into<Image<'a>>, text: impl Into<WidgetText>) -> Self {
        Self::opt_image_and_text(Some(image.into()), Some(text.into()))
    }

    pub fn opt_image_and_text(image: Option<Image<'a>>, text: Option<WidgetText>) -> Self {
        Self {
            text,
            image,
            right_text: Default::default(),
            wrap_mode: None,
            fill: None,
            stroke: None,
            sense: Sense::click(),
            small: false,
            frame: None,
            min_size: Vec2::ZERO,
            corner_radius: None,
            selected: false,
            image_tint_follows_text_color: false,
        }
    }

    /// Set the wrap mode for the text.
    ///
    /// By default, [`crate::Ui::wrap_mode`] will be used, which can be overridden with [`crate::Style::wrap_mode`].
    ///
    /// Note that any `\n` in the text will always produce a new line.
    #[inline]
    pub fn wrap_mode(mut self, wrap_mode: TextWrapMode) -> Self {
        self.wrap_mode = Some(wrap_mode);
        self
    }

    /// Set [`Self::wrap_mode`] to [`TextWrapMode::Wrap`].
    #[inline]
    pub fn wrap(mut self) -> Self {
        self.wrap_mode = Some(TextWrapMode::Wrap);

        self
    }

    /// Set [`Self::wrap_mode`] to [`TextWrapMode::Truncate`].
    #[inline]
    pub fn truncate(mut self) -> Self {
        self.wrap_mode = Some(TextWrapMode::Truncate);
        self
    }

    /// Override background fill color. Note that this will override any on-hover effects.
    /// Calling this will also turn on the frame.
    #[inline]
    pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
        self.fill = Some(fill.into());
        self.frame = Some(true);
        self
    }

    /// Override button stroke. Note that this will override any on-hover effects.
    /// Calling this will also turn on the frame.
    #[inline]
    pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stroke = Some(stroke.into());
        self.frame = Some(true);
        self
    }

    /// Make this a small button, suitable for embedding into text.
    #[inline]
    pub fn small(mut self) -> Self {
        if let Some(text) = self.text {
            self.text = Some(text.text_style(TextStyle::Body));
        }
        self.small = true;
        self
    }

    /// Turn off the frame
    #[inline]
    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = Some(frame);
        self
    }

    /// By default, buttons senses clicks.
    /// Change this to a drag-button with `Sense::drag()`.
    #[inline]
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }

    /// Set the minimum size of the button.
    #[inline]
    pub fn min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    /// Set the rounding of the button.
    #[inline]
    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(corner_radius.into());
        self
    }

    #[inline]
    #[deprecated = "Renamed to `corner_radius`"]
    pub fn rounding(self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius(corner_radius)
    }

    /// If true, the tint of the image is multiplied by the widget text color.
    ///
    /// This makes sense for images that are white, that should have the same color as the text color.
    /// This will also make the icon color depend on hover state.
    ///
    /// Default: `false`.
    #[inline]
    pub fn image_tint_follows_text_color(mut self, image_tint_follows_text_color: bool) -> Self {
        self.image_tint_follows_text_color = image_tint_follows_text_color;
        self
    }

    /// Show some text on the right side of the button, in weak color.
    ///
    /// Designed for menu buttons, for setting a keyboard shortcut text (e.g. `Ctrl+S`).
    ///
    /// The text can be created with [`crate::Context::format_shortcut`].
    ///
    /// See also [`Self::right_text`].
    #[inline]
    pub fn shortcut_text(mut self, shortcut_text: impl Into<WidgetText>) -> Self {
        self.right_text = shortcut_text.into().weak();
        self
    }

    /// Show some text on the right side of the button.
    #[inline]
    pub fn right_text(mut self, right_text: impl Into<WidgetText>) -> Self {
        self.right_text = right_text.into();
        self
    }

    /// If `true`, mark this button as "selected".
    #[inline]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Widget for Button<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let Button {
            text,
            image,
            right_text,
            wrap_mode,
            fill,
            stroke,
            sense,
            small,
            frame,
            min_size,
            corner_radius,
            selected,
            image_tint_follows_text_color,
        } = self;

        let frame = frame.unwrap_or_else(|| ui.visuals().button_frame);

        let default_font_height = || {
            let font_selection = FontSelection::default();
            let font_id = font_selection.resolve(ui.style());
            ui.fonts(|f| f.row_height(&font_id))
        };

        let text_font_height = ui
            .fonts(|fonts| text.as_ref().map(|wt| wt.font_height(fonts, ui.style())))
            .unwrap_or_else(default_font_height);

        let mut button_padding = if frame {
            ui.spacing().button_padding
        } else {
            Vec2::ZERO
        };
        if small {
            button_padding.y = 0.0;
        }

        let (space_available_for_image, right_text_font_height) = if let Some(text) = &text {
            let font_height = ui.fonts(|fonts| text.font_height(fonts, ui.style()));
            (
                Vec2::splat(font_height), // Reasonable?
                font_height,
            )
        } else {
            (
                (ui.available_size() - 2.0 * button_padding).at_least(Vec2::ZERO),
                default_font_height(),
            )
        };

        let image_size = if let Some(image) = &image {
            image
                .load_and_calc_size(ui, space_available_for_image)
                .unwrap_or(space_available_for_image)
        } else {
            Vec2::ZERO
        };

        let gap_before_right_text = ui.spacing().item_spacing.x;

        let mut text_wrap_width = ui.available_width() - 2.0 * button_padding.x;
        if image.is_some() {
            text_wrap_width -= image_size.x + ui.spacing().icon_spacing;
        }

        // Note: we don't wrap the right text
        let right_galley = (!right_text.is_empty()).then(|| {
            right_text.into_galley(
                ui,
                Some(TextWrapMode::Extend),
                f32::INFINITY,
                TextStyle::Button,
            )
        });

        if let Some(right_galley) = &right_galley {
            // Leave space for the right text:
            text_wrap_width -= gap_before_right_text + right_galley.size().x;
        }

        let galley =
            text.map(|text| text.into_galley(ui, wrap_mode, text_wrap_width, TextStyle::Button));

        let mut desired_size = Vec2::ZERO;
        if image.is_some() {
            desired_size.x += image_size.x;
            desired_size.y = desired_size.y.max(image_size.y);
        }
        if image.is_some() && galley.is_some() {
            desired_size.x += ui.spacing().icon_spacing;
        }
        if let Some(galley) = &galley {
            desired_size.x += galley.size().x;
            desired_size.y = desired_size.y.max(galley.size().y).max(text_font_height);
        }
        if let Some(right_galley) = &right_galley {
            desired_size.x += gap_before_right_text + right_galley.size().x;
            desired_size.y = desired_size
                .y
                .max(right_galley.size().y)
                .max(right_text_font_height);
        }
        desired_size += 2.0 * button_padding;
        if !small {
            desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
        }
        desired_size = desired_size.at_least(min_size);

        let (rect, mut response) = ui.allocate_at_least(desired_size, sense);
        response.widget_info(|| {
            let mut widget_info = WidgetInfo::new(WidgetType::Button);
            widget_info.enabled = ui.is_enabled();

            if let Some(galley) = &galley {
                widget_info.label = Some(galley.text().to_owned());
            } else if let Some(image) = &image {
                widget_info.label = image.alt_text.clone();
            }
            widget_info
        });

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response);

            let (frame_expansion, frame_cr, frame_fill, frame_stroke) = if selected {
                let selection = ui.visuals().selection;
                (
                    Vec2::ZERO,
                    CornerRadius::ZERO,
                    selection.bg_fill,
                    selection.stroke,
                )
            } else if frame {
                let expansion = Vec2::splat(visuals.expansion);
                (
                    expansion,
                    visuals.corner_radius,
                    visuals.weak_bg_fill,
                    visuals.bg_stroke,
                )
            } else {
                Default::default()
            };
            let frame_cr = corner_radius.unwrap_or(frame_cr);
            let frame_fill = fill.unwrap_or(frame_fill);
            let frame_stroke = stroke.unwrap_or(frame_stroke);
            ui.painter().rect(
                rect.expand2(frame_expansion),
                frame_cr,
                frame_fill,
                frame_stroke,
                epaint::StrokeKind::Inside,
            );

            let mut cursor_x = rect.min.x + button_padding.x;

            if let Some(image) = &image {
                let mut image_pos = ui
                    .layout()
                    .align_size_within_rect(image_size, rect.shrink2(button_padding))
                    .min;
                if galley.is_some() || right_galley.is_some() {
                    image_pos.x = cursor_x;
                }
                let image_rect = Rect::from_min_size(image_pos, image_size);
                cursor_x += image_size.x;
                let tlr = image.load_for_size(ui.ctx(), image_size);
                let mut image_options = image.image_options().clone();
                if image_tint_follows_text_color {
                    image_options.tint = image_options.tint * visuals.text_color();
                }
                widgets::image::paint_texture_load_result(
                    ui,
                    &tlr,
                    image_rect,
                    image.show_loading_spinner,
                    &image_options,
                    None,
                );
                response = widgets::image::texture_load_result_response(
                    &image.source(ui.ctx()),
                    &tlr,
                    response,
                );
            }

            if image.is_some() && galley.is_some() {
                cursor_x += ui.spacing().icon_spacing;
            }

            if let Some(galley) = galley {
                let mut text_pos = ui
                    .layout()
                    .align_size_within_rect(galley.size(), rect.shrink2(button_padding))
                    .min;
                if image.is_some() || right_galley.is_some() {
                    text_pos.x = cursor_x;
                }
                ui.painter().galley(text_pos, galley, visuals.text_color());
            }

            if let Some(right_galley) = right_galley {
                // Always align to the right
                let layout = if ui.layout().is_horizontal() {
                    ui.layout().with_main_align(Align::Max)
                } else {
                    ui.layout().with_cross_align(Align::Max)
                };
                let right_text_pos = layout
                    .align_size_within_rect(right_galley.size(), rect.shrink2(button_padding))
                    .min;

                ui.painter()
                    .galley(right_text_pos, right_galley, visuals.text_color());
            }
        }

        if let Some(cursor) = ui.visuals().interact_cursor {
            if response.hovered() {
                ui.ctx().set_cursor_icon(cursor);
            }
        }

        response
    }
}
