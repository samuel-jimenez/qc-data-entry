extern crate native_windows_derive as nwd;

use nwd::NwgPartial;
use nwg::taffy::FlexDirection;
use qc_data_entry_derive::derive_mass;

use crate::{
    constants::{COL_20, COL_80, GROUP_PADDING, VISUAL_MARGIN},
    NumberUnitsEdit, RangeView,
};

#[derive(Default, NwgPartial)]
pub struct OBPanelView {
    #[nwg_layout(auto_spacing:None)]
    frame_layout: nwg::FlexboxLayout,

    #[nwg_control(flags: "VISIBLE")]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_80)]
    wb_frame: nwg::Frame,
    #[nwg_partial(parent: wb_frame)]
    product_wb: OBProductView,

    // #[nwg_partial(be: control)]
    #[nwg_partial_control]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_20)]
    product_range: OBRangesView,
}

#[derive_mass]
#[derive(Default, NwgPartial)]
pub struct OBProductView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout, margin:VISUAL_MARGIN)]
    visual: nwg::CheckBox,

    mass: nwg::LabeledEdit,
}

#[derive(Default, NwgPartial)]
pub struct OBRangesView {
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
    mass: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    sg: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    density: RangeView,
}
