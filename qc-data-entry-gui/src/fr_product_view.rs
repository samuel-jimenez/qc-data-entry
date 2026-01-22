extern crate native_windows_derive as nwd;

use nwd::NwgPartial;
use nwg::taffy::FlexDirection;
use qc_data_entry_derive::derive_mass;

use crate::{
    constants::{COL_20, COL_40, GROUP_PADDING, VISUAL_MARGIN},
    NumberUnitsEdit, RangeView,
};

#[derive(Default, NwgPartial)]
pub struct FRPanelView {
    // Layout
    // #[nwg_layout]
    #[nwg_layout(auto_spacing:None)]
    // #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Top")]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_40)]
    group_top: nwg::GroupBox,
    #[nwg_partial(parent: group_top)]
    product_top: FRProductView,

    #[nwg_control(text: "Btm")]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_40)]
    group_btm: nwg::GroupBox,
    #[nwg_partial(parent: group_btm)]
    product_btm: FRProductView,

    #[nwg_control]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_20)]
    group_range: nwg::Frame,
    #[nwg_partial(parent: group_range)]
    product_range: FRRangesView,
}

impl FRPanelView {
    pub fn click(&self) {
        println!("{}", self.product_top.click());
        self.product_range
            .sg
            // .set(&(vec![None, Some(5.1), None]).into());
            // .set(&vec![None, Some(5.1), None].into());
            .set(&vec![Some(0.1), Some(5.1), Some(0.1115)].into());
        // .set(vec![None, Some(5.1), None].into());
        // .set(*vec![None, Some(5.1), None].into());
    }
}

#[derive_mass]
#[derive(Default, NwgPartial)]
pub struct FRProductView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout,margin:VISUAL_MARGIN)]
    visual: nwg::CheckBox,

    #[nwg_control(label: "Viscosity")]
    #[nwg_layout_item(layout: frame_layout)]
    viscosity: nwg::LabeledEdit,

    mass: nwg::LabeledEdit,

    #[nwg_control(label: "String")]
    #[nwg_layout_item(layout: frame_layout)]
    string: nwg::LabeledEdit,
}

// macro_rules! unwrap_or_return {
//     ( $e:expr ) => {
//         match $e {
//             Ok(x) => x,
//             Err(_) => return,
//         }
//     };
// }
// pub(crate) use unwrap_or_return;

impl FRProductView {
    fn click(&self) -> String {
        self.viscosity.text()
    }
}

#[derive(Default, NwgPartial)]
pub struct FRRangesView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout,margin:VISUAL_MARGIN)]
    visual: nwg::Label,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    viscosity: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    mass: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    string: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    sg: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    density: RangeView,
}
