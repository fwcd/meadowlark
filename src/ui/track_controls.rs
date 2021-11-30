use vizia::*;

use crate::state::ui_state::TimelineTrackUiState;

pub fn track_controls<D>(cx: &mut Context, track_data: D)
where
    D: 'static + DataHandle<Data = TimelineTrackUiState>,
{
    HStack::new(cx, move |cx| {
        // Track Controls
        HStack::new(cx, move |cx| {
            // Track color
            Element::new(cx).width(Pixels(10.0)).background_color(Color::rgb(254, 64, 64));
            VStack::new(cx, move |cx| {
                HStack::new(cx, move |cx| {
                    let track_data = track_data.get(cx).clone();

                    Label::new(cx, &track_data.name);
                    // Record Button
                    Button::new(cx, |_| {}, |_| {}).width(Pixels(30.0)).height(Pixels(30.0));
                    // Solo Button
                    Button::new(cx, |_| {}, |_| {}).width(Pixels(30.0)).height(Pixels(30.0));
                    // Mute Button
                    Button::new(cx, |_| {}, |_| {}).width(Pixels(30.0)).height(Pixels(30.0));
                });
            })
            .background_color(Color::rgb(179, 172, 174));
        });
        // Clips
    })
    .height(Pixels(100.0));
}
