use epaint::{Color32, Pos2, Rect, Shape, Stroke, emath::lerp, vec2};

use crate::{Response, Sense, Ui, Widget, WidgetInfo, WidgetType};

/// A spinner widget used to indicate loading.
///
/// See also: [`crate::ProgressBar`].
#[must_use = "You should put this widget in a ui with `ui.add(widget);`"]
#[derive(Default)]
pub struct Spinner {
    /// Uses the style's `interact_size` if `None`.
    size: Option<f32>,
    color: Option<Color32>,
}

impl Spinner {
    /// Create a new spinner that uses the style's `interact_size` unless changed.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the spinner's size. The size sets both the height and width, as the spinner is always
    /// square. If the size isn't set explicitly, the active style's `interact_size` is used.
    #[inline]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the spinner's color.
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Paint the spinner in the given rectangle.
    pub fn paint_at(&self, ui: &Ui, rect: Rect) {
        if ui.is_rect_visible(rect) {
            ui.ctx().request_repaint(); // because it is animated

            let color = self
                .color
                .unwrap_or_else(|| ui.visuals().strong_text_color());
            let radius = (rect.height() / 2.0) - 2.0;
            let n_points = (radius.round() as u32).clamp(8, 128);
            let time = ui.input(|i| i.time);
            let start_angle = time * std::f64::consts::TAU;
            let end_angle = start_angle + 240f64.to_radians() * time.sin();
            let points: Vec<Pos2> = (0..n_points)
                .map(|i| {
                    let angle = lerp(start_angle..=end_angle, i as f64 / n_points as f64);
                    let (sin, cos) = angle.sin_cos();
                    rect.center() + radius * vec2(cos as f32, sin as f32)
                })
                .collect();
            ui.painter()
                .add(Shape::line(points, Stroke::new(3.0, color)));
        }
    }
}

impl Widget for Spinner {
    fn ui(self, ui: &mut Ui) -> Response {
        let size = self
            .size
            .unwrap_or_else(|| ui.style().spacing.interact_size.y);
        let (rect, response) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
        response.widget_info(|| WidgetInfo::new(WidgetType::ProgressIndicator));
        self.paint_at(ui, rect);

        response
    }
}
