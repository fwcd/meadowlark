use vizia::*;

use super::timeline_view::TimelineViewState;

use femtovg::{Align, Baseline, Paint, Path};
pub struct TimelineGrid {}

impl TimelineGrid {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build2(cx, |cx| {})
    }
}

impl View for TimelineGrid {
    fn draw(&self, cx: &Context, canvas: &mut Canvas) {
        if let Some(timeline_view) = cx.data::<TimelineViewState>() {
            let start_time = timeline_view.start_time;
            let end_time = timeline_view.end_time;
            let timeline_start = timeline_view.timeline_start;
            let timeline_end = timeline_view.timeline_end;
            let timeline_width = timeline_view.width;

            let bounds = cx.cache.get_bounds(cx.current);

            let font = cx.style.borrow().font.get(cx.current).cloned().unwrap_or_default();

            let default_font = cx
                .resource_manager
                .fonts
                .get(&cx.style.borrow().default_font)
                .and_then(|font| match font {
                    FontOrId::Id(id) => Some(id),
                    _ => None,
                })
                .expect("Failed to find default font");

            let font_id = cx
                .resource_manager
                .fonts
                .get(&font)
                .and_then(|font| match font {
                    FontOrId::Id(id) => Some(id),
                    _ => None,
                })
                .unwrap_or(default_font);

            canvas.save();

            canvas.scissor(bounds.x, bounds.y, bounds.w, bounds.h);

            for i in (start_time.0.ceil() as usize)..(end_time.0.ceil() as usize) {
                let ratio = (i as f64 - start_time.0) / (end_time.0 - start_time.0);
                let mut path = Path::new();
                path.move_to(bounds.x + (ratio as f32 * timeline_width).floor(), bounds.y);
                path.line_to(
                    bounds.x + (ratio as f32 * timeline_width).floor(),
                    bounds.y + bounds.h,
                );
                canvas.stroke_path(&mut path, Paint::color(femtovg::Color::rgb(36, 36, 36)));
                let mut text_paint = Paint::color(femtovg::Color::rgb(255, 255, 255));
                text_paint.set_font(&[font_id.clone()]);
                text_paint.set_text_align(Align::Left);
                text_paint.set_text_baseline(Baseline::Top);
                canvas.fill_text(
                    bounds.x + (ratio as f32 * timeline_width).floor() + 2.0,
                    bounds.y,
                    &(i + 1).to_string(),
                    text_paint,
                );
            }

            canvas.restore();
        }
    }
}
