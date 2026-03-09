extern crate native_windows_derive as nwd;

use nwd::NwgPartial;
use nwg::{taffy::FlexDirection, KeyPress, ModifierKeys};
use qc_data_entry_derive::derive_mass;

use crate::{
    constants::{COL_20, COL_80, GROUP_PADDING, VISUAL_MARGIN},
    number_edit_fixed::FixedNumEdit,
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
    #[nwg_shortcuts((visual, mass)  [A, D, NumpadSlash, NumpadTimes]: [OBPanelView::proc_nav_shortcut()])]
    product_wb: OBProductView,

    // #[nwg_partial(be: control)]
    #[nwg_partial_control]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_20)]
    product_range: OBRangesView,
}
impl OBPanelView {
    fn proc_nav_shortcut() {}
}

#[derive_mass]
#[derive(Default, NwgPartial)]
#[nwg_shortcuts((visual, mass)  [W, S, NumpadMinus, NumpadPlus]: [OBProductView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
pub struct OBProductView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout, margin:VISUAL_MARGIN)]
    visual: nwg::CheckBox,

    mass: FixedNumEdit,
}
impl OBProductView {
    fn proc_nav_shortcut(&self, combo: &nwg::KeyCombo, handle: &nwg::ControlHandle) {
        match combo {
            nwg::KeyCombo {
                key: KeyPress::W, ..
            }
            | nwg::KeyCombo {
                key: KeyPress::NumpadMinus,
                ..
            } => {
                if handle == &self.visual {
                    self.visual.set_focus()
                } else if handle == &self.mass {
                    self.visual.set_focus()
                }
            }
            nwg::KeyCombo {
                key: KeyPress::S, ..
            }
            | nwg::KeyCombo {
                key: KeyPress::NumpadPlus,
                ..
            } => {
                if handle == &self.visual {
                    self.mass.set_focus()
                } else if handle == &self.mass {
                    self.mass.set_focus()
                }
            }
            _ => {}
        }
        println!("woo");
    }
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
