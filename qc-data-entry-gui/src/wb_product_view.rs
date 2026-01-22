extern crate native_windows_derive as nwd;

use nwd::NwgPartial;
use nwg::taffy::FlexDirection;

use crate::{
    constants::{COL_20, COL_80, GROUP_PADDING},
    RangeView, VISUAL_MARGIN,
};

#[derive(Default, NwgPartial)]
pub struct WBPanelView {
    #[nwg_layout(auto_spacing:None)]
    wb_layout: nwg::FlexboxLayout,

    #[nwg_control(flags: "VISIBLE")]
    // #[nwg_layout_item(layout: wb_layout)]
    #[nwg_layout_item(layout: wb_layout, size:COL_80)]
    wb_frame: nwg::Frame,
    #[nwg_partial(parent: wb_frame)]
    product_wb: WBProductView,

    // #[nwg_partial(be: control)]
    #[nwg_partial_control]
    // #[nwg_layout_item(layout: wb_layout)]
    #[nwg_layout_item(layout: wb_layout, size:COL_20)]
    product_range: WBRangesView,
}

#[derive(Default, NwgPartial)]
pub struct WBProductView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout, margin:VISUAL_MARGIN)]
    visual: nwg::CheckBox,

    #[nwg_control(label: "pH")]
    #[nwg_layout_item(layout: frame_layout)]
    #[nwg_events(OnTextInput: [WBProductView::tick])]
    ph: nwg::LabeledEdit,

    #[nwg_control(label: "Specific Gravity")]
    #[nwg_layout_item(layout: frame_layout)]
    sg: nwg::LabeledEdit,
}

impl WBProductView {
    fn tick(&self) {
        self.sg.set_label(&self.ph.text());
    }
}

#[derive(Default, NwgPartial)]
pub struct WBRangesView {
    #[nwg_control]
    #[nwg_root]
    frame: nwg::Frame,

    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout,margin:VISUAL_MARGIN)]
    visual: nwg::Label,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    ph: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    sg: RangeView,
}
