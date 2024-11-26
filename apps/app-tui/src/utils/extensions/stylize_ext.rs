use ratatui::style::Stylize;
use ratatui::prelude::*;

use paste::paste;
use ratatui::Frame;
use ratatui::widgets::WidgetRef;

macro_rules! modifier {
    ( $modifier:ident ) => {
        paste! {
            #[doc = "Adds or removes the [`" $modifier:upper "`](Modifier::" $modifier:upper ") modifier."]
            #[must_use = concat!("`", stringify!($modifier), "` returns the modified style without modifying the original")]
            fn [<with_$modifier>](self, enabled: bool) -> T {
                if enabled {
                    self.add_modifier(Modifier::[<$modifier:upper>])
                } else {
                    self.remove_modifier(Modifier::[<$modifier:upper>])
                }
            }
        }
    };

    ( $modifier:ident, $name:ident ) => {
        paste! {
            #[doc = "Adds or removes the [`" $modifier:upper "`](Modifier::" $modifier:upper ") modifier."]
            #[must_use = concat!("`", stringify!($modifier), "` returns the modified style without modifying the original")]
            fn [<with_$name>](self, enabled: bool) -> T {
                if enabled {
                    self.add_modifier(Modifier::[<$modifier:upper>])
                } else {
                    self.remove_modifier(Modifier::[<$modifier:upper>])
                }
            }
        }
    };
}

pub trait StylizeExt<'a, T>: Stylize<'a, T> {

    modifier!(bold);
    modifier!(dim);
    modifier!(italic);
    modifier!(underlined);
    modifier!(slow_blink);
    modifier!(rapid_blink);
    modifier!(reversed);
    modifier!(hidden);
    modifier!(crossed_out);
    modifier!(crossed_out, strikethrough);

    #[must_use = "`with_fg` returns the modified style without modifying the original"]
    fn with_fg<C: Into<Color>>(self, boolean: bool, first: C, second: C) -> T {
        self.fg(if boolean { first } else { second })
    }

    #[must_use = "`with_bg` returns the modified style without modifying the original"]
    fn with_bg<C: Into<Color>>(self, boolean: bool, first: C, second: C) -> T {
        self.bg(if boolean { first } else { second })
    }
}

impl<'a, T, A: Stylize<'a, T>> StylizeExt<'a, T> for A {}

pub trait WidgetExt: WidgetRef {
    fn render_widget(&self, frame: &mut Frame, area: Rect) {
        self.render_ref(area, frame.buffer_mut());
    }
}

impl<A: WidgetRef> WidgetExt for A {}