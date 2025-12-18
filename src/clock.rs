// #![windows_subsystem = "windows"]
/*!
 *   A very simple application that shows your name in a message box.
 *   Unlike `basic_d`, this example uses layout to position the controls in the window
 */

extern crate native_windows_derive as nwd;

use std::cell::RefCell;
use std::cmp::min;
use std::io::Write;
use std::rc::Rc;
// use std::time;

// use chrono::{DateTime, Local, TimeDelta, Utc,seconds};
use chrono::{DateTime, TimeDelta, Utc};
use nwd::{NwgPartial, NwgUi};
use nwg::{CharEffects, CharFormat, EventData, NativeUi};

// Stretch style
use nwg::stretch::{
    geometry::{Rect, Size},
    style::{Dimension as D, FlexDirection, FlexWrap},
};

const PC_50: D = D::Percent(0.5);
const PC_33: D = D::Percent(0.33);
const PC_100: D = D::Percent(1.0);

const PT_10: D = D::Points(10.0);
const PT_5: D = D::Points(5.0);
const PADDING: Rect<D> = Rect {
    start: PT_10,
    end: PT_10,
    top: PT_10,
    bottom: PT_10,
};
const MARGIN: Rect<D> = Rect {
    start: PT_5,
    end: PT_5,
    top: PT_5,
    bottom: PT_5,
};

#[derive(Default, NwgPartial)]
pub struct ClockBox {
    width: RefCell<u32>,
    height: RefCell<u32>,

    // Refresh timer (30 FPS)
    #[nwg_control(interval: std::time::Duration::from_millis(1000), active: true)]
    #[nwg_events(OnTimerTick: [ClockBox::tick])]
    refresh_timer: nwg::AnimationTimer,

    // Layout
    #[nwg_layout( flex_direction: FlexDirection::Row, flex_wrap:FlexWrap::NoWrap,    aspect_ratio: stretch::number::Number::Defined(3.),)]
    window_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "00")]
    #[nwg_layout_item(layout: window_layout)]
    time_past: nwg::RichLabel,

    #[nwg_control(text: "00")]
    #[nwg_layout_item(layout: window_layout)]
    time_now: nwg::RichLabel,

    #[nwg_control(text: "00")]
    #[nwg_layout_item(layout: window_layout)]
    time_future: nwg::RichLabel,
}
impl ClockBox {
    fn init(&self) {
        // TODO use GetTextExtentPoint32
        const ASPECT_W: u32 = 5;
        const ASPECT_H: u32 = 2;

        const SCALE_NUM: u32 = 14;
        const SCALE_DEN: u32 = 11;

        let font_height: i32 = (min(
            *self.height.borrow() * ASPECT_W,
            *self.width.borrow() * ASPECT_H,
        ) * SCALE_NUM
            / SCALE_DEN)
            .try_into()
            .unwrap();

        self.time_past.set_char_format(
            0..2,
            &CharFormat {
                effects: Some(CharEffects::BOLD),
                height: Some(font_height),
                y_offset: Some(-font_height / 2),
                text_color: Some([200, 0, 0]),
                ..Default::default()
            },
        );

        self.time_now.set_char_format(
            0..2,
            &CharFormat {
                effects: Some(CharEffects::BOLD),
                height: Some(font_height),
                // y_offset: Some(font_height/2), //TODO this doesnt work
                text_color: Some([0, 200, 0]),
                ..Default::default()
            },
        );
        self.time_future.set_char_format(
            0..2,
            &CharFormat {
                effects: Some(CharEffects::BOLD),
                height: Some(font_height),
                y_offset: Some(-font_height / 2),
                text_color: Some([0, 0, 200]),
                ..Default::default()
            },
        );
    }

    pub fn set_size(&self, w: u32, h: u32) {
        *self.width.borrow_mut() = w;
        *self.height.borrow_mut() = h;
        self.init();
    }

    fn tick(&self) {
        let run_time = Utc::now();
        let future_time = run_time
            .checked_add_signed(TimeDelta::seconds(20))
            .unwrap_or(DateTime::UNIX_EPOCH);
        self.time_now
            .set_text(&run_time.time().format("%S").to_string());
        self.time_future
            .set_text(&future_time.time().format("%S").to_string());
        self.init();
    }
}
