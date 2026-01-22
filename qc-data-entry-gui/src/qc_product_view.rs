extern crate native_windows_derive as nwd;

use nwd::NwgPartial;

#[derive(Default, NwgPartial)]
pub struct QcProductView {
    // Layout
    #[nwg_layout( flex_direction: taffy::FlexDirection::Row,  auto_spacing:None)]
    window_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: window_layout)]
    visual: nwg::CheckBox,
}
