extern crate native_windows_derive as nwd;

use std::cell::Cell;

// use nwd::{NwgControl, NwgPartial};
use nwd::NwgPartial;
use nwg::{subclass_layout, taffy::FlexDirection};

// #[derive(Default, Clone)]
// #[derive(Default, Clone, Copy)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Range {
    min: Option<f32>,
    target: Option<f32>,
    max: Option<f32>,
    // precision: int,//TODO allow specify precision
}
// {:.*}",   5, 0.01);
impl Range {
    // pub fn check(&self, val: f32) -> bool {
    //     self.min.is_none_or(|x| x <= val) && self.max.is_none_or(|x| x >= val)
    // }

    pub fn check_min(&self, val: f32) -> bool {
        self.min.is_none_or(|x| x <= val)
    }
    pub fn check_max(&self, val: f32) -> bool {
        self.max.is_none_or(|x| x >= val)
    }
}

impl From<Vec<Option<f32>>> for Range {
    fn from(val_in: Vec<Option<f32>>) -> Self {
        Self {
            min: val_in[0],
            target: val_in[1],
            max: val_in[2],
        }
    }
}

#[derive(Default, NwgPartial)]
// #[derive(Default, NwgControl)]
pub struct RangeView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Row)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "0.00")]
    // #[nwg_control(text: "")]
    #[nwg_layout_item(layout: frame_layout)]
    min: nwg::Label,

    #[nwg_control(text: "≤")]
    // #[nwg_control(text: "")] //≤
    #[nwg_layout_item(layout: frame_layout)]
    min_spacer: nwg::Label,

    #[nwg_control(text: "0.00")]
    // #[nwg_control(text: "")]
    #[nwg_layout_item(layout: frame_layout)]
    target: nwg::Label,

    #[nwg_control(text: "≤")]
    // #[nwg_control(text: "")] //≤
    #[nwg_layout_item(layout: frame_layout)]
    max_spacer: nwg::Label,

    #[nwg_control(text: "0.00")]
    // #[nwg_control(text: "")]
    #[nwg_layout_item(layout: frame_layout)]
    max: nwg::Label,

    range: Cell<Range>,

    min_ok_p: Cell<bool>,
    max_ok_p: Cell<bool>,
}

subclass_layout!(RangeView, FlexboxLayout, frame_layout);

//TODO allow specify precision
impl RangeView {
    pub fn set(&self, val: Range) {
        let (min, min_spacer) = match val.min {
            // Some(x) => (format!("{:.3}", x).as_str(), "≤"),
            Some(x) => (&format!("{:.3}", x), "≤"),
            None => (&"".to_string(), ""),
        };
        let target = match val.target {
            Some(x) => &format!("{:.3}", x),
            None => "",
        };
        let (max, max_spacer) = match val.max {
            // Some(x) => (format!("{:.3}", x).as_str(), "≤"),
            Some(x) => (&format!("{:.3}", x), "≤"),
            None => (&"".to_string(), ""),
        };

        self.min.set_text(min);
        self.min_spacer.set_text(min_spacer);
        self.target.set_text(target);
        self.max.set_text(max);
        self.max_spacer.set_text(max_spacer);
        self.range.set(val);
        self.min_ok_p.set(true);
        self.max_ok_p.set(true);
    }

    pub fn check(&self, val: f32) -> bool {
        // if  self.range.get(){}
        // let foo = self.range.get();
        // let foo = self.range.get_mut();
        // if self.range.get_mut() {}
        // let range = self.range.get_mut();
        // let range = self.range.get();
        // let range = self.range.get().check();
        // c.as_ptr();

        // range.min.is_none_or(|x| x <= val) && range.max.is_none_or(|x| x >= val)
        // *self.range.as_ptr().check()
        // let range = *self.range.as_ptr()
        // (*self.range.as_ptr()).check(val)
        // let ok_p = self.range.get().check(val);

        let range = self.range.get();
        let min_ok_p = range.check_min(val);
        if min_ok_p != self.min_ok_p.get() {
            if min_ok_p {
                self.min.set_border_color(None);
            } else {
                self.min.set_border_color(Some([0xff, 0, 0]));
            }
        }

        let max_ok_p = range.check_max(val);
        if max_ok_p != self.max_ok_p.get() {
            if max_ok_p {
                self.max.set_border_color(None);
            } else {
                self.max.set_border_color(Some([0xff, 0, 0]));
            }
        }
        self.min_ok_p.set(min_ok_p);
        self.max_ok_p.set(max_ok_p);

        //     println!("RangeView check val {val} {}", ok_p);
        // // if ok_p
        // ok_p
        min_ok_p && max_ok_p
    }
}

#[derive(Default, NwgPartial)]
pub struct RangeEdit {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Row)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    // #[nwg_control]
    #[nwg_control(text: "0.00")]
    #[nwg_layout_item(layout: frame_layout)]
    min: nwg::TextInput,

    #[nwg_control]
    #[nwg_layout_item(layout: frame_layout)]
    target: nwg::TextInput,

    #[nwg_control]
    #[nwg_layout_item(layout: frame_layout)]
    max: nwg::TextInput,
}
