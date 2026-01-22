extern crate native_windows_derive as nwd;

// use nwd::{NwgControl, NwgPartial};
use nwd::NwgPartial;
use nwg::{subclass_layout, taffy::FlexDirection};

#[derive(Default)]
pub struct Range {
    min: Option<f32>,
    target: Option<f32>,
    max: Option<f32>,
    // precision: int,//TODO allow specify precision
}
// {:.*}",   5, 0.01);

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
}

subclass_layout!(RangeView, FlexboxLayout, frame_layout);

//TODO allow specify precision
impl RangeView {
    pub fn set(&self, val: &Range) {
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
