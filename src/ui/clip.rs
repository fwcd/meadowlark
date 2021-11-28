
use vizia::*;


struct Clip {

}

impl Clip {
    pub fn new(cx: &mut Context) -> Self {
        Self {

        }.build2(cx, |cx|{
            Label::new(cx, &clip_name).height(Pixels(20.0)).width(Stretch(1.0)).background_color(Color::rgb(254, 64, 64));
            Element::new(cx).background_color(Color::rgba(242, 77, 66, 15));
        })
    }
}

impl View for Clip {

}