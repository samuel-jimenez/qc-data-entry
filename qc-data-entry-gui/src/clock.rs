extern crate native_windows_derive as nwd;

use std::{cell::RefCell, cmp::min};

use chrono::{DateTime, TimeDelta, Utc};
use nwd::NwgPartial;
use nwg::{
    taffy::{style::FlexDirection, LengthPercentageAuto},
    CharEffects, CharFormat,
};
//e HideCaret
#[derive(Default, NwgPartial)]
pub struct ClockBox {
    height: RefCell<i32>,

    // Refresh timer (30 FPS)
    #[nwg_control(interval: std::time::Duration::from_millis(1000), active: true)]
    #[nwg_events(OnTimerTick: [ClockBox::tick])]
    refresh_timer: nwg::AnimationTimer,

    // Resources
    #[nwg_resource(family: "MS Shell Dlg 2")]
    font: nwg::Font,

    // Layout
    #[nwg_layout( flex_direction: FlexDirection::Row,  auto_spacing:None)]
    window_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "00",font: Some(&data.font))]
    #[nwg_layout_item(layout: window_layout)]
    time_past: nwg::RichLabel,

    #[nwg_control(text: "00", font: Some(&data.font))]
    #[nwg_layout_item(layout: window_layout)]
    time_now: nwg::RichLabel,

    #[nwg_control(text: "00", font: Some(&data.font))]
    #[nwg_layout_item(layout: window_layout)]
    time_future: nwg::RichLabel,
}
impl ClockBox {
    fn format(&self) {
        let mut fmt = CharFormat {
            effects: Some(CharEffects::BOLD),
            height: Some(*self.height.borrow()),
            text_color: Some([0, 200, 0]),
            ..Default::default()
        };
        self.time_now.set_char_format_all(&fmt);

        fmt.text_color = Some([200, 0, 0]);
        fmt.y_offset = Some(*self.height.borrow() / -2);
        self.time_past.set_char_format_all(&fmt);

        fmt.text_color = Some([0, 0, 200]);
        self.time_future.set_char_format_all(&fmt);
    }

    pub fn set_size(&self, w: u32, h: u32) {
        // TODO use GetTextExtentPoint32

        const ASPECT_W: u32 = 5;
        const ASPECT_H: u32 = 2;
        const SCALE: f32 = 10.0;
        let height_nominal = min(h, w * ASPECT_H / ASPECT_W) as f32;

        *self.height.borrow_mut() = (height_nominal * SCALE) as i32;
        self.window_layout
            .modify_child_style(self.time_now.handle, |s| {
                s.margin.top = LengthPercentageAuto::length(-0.1 * height_nominal)
            });
        self.format();
    }

    fn tick(&self) {
        let now = Utc::now();

        self.time_now.set_text(&now.time().format("%S").to_string());
        self.time_past.set_text(
            &now.checked_add_signed(TimeDelta::seconds(-20))
                .unwrap_or(DateTime::UNIX_EPOCH)
                .time()
                .format("%S")
                .to_string(),
        );
        self.time_future.set_text(
            &now.checked_add_signed(TimeDelta::seconds(20))
                .unwrap_or(DateTime::UNIX_EPOCH)
                .time()
                .format("%S")
                .to_string(),
        );
        self.format();
    }
}
