extern crate native_windows_derive as nwd;

use nwd::NwgPartial;
use nwg::{taffy::FlexDirection, ControlHandle, KeyPress, ModifierKeys};
use qc_data_entry_derive::derive_mass;

use crate::{
    constants::{COL_20, COL_30, COL_70, COL_80, GROUP_PADDING, VISUAL_MARGIN},
    number_edit_fixed::FixedNumEdit,
    NumberUnitsEdit, RangeView,
};

#[derive(Default, NwgPartial)]
pub struct OBPanelView {
    #[nwg_layout(auto_spacing:None)]
    frame_layout: nwg::FlexboxLayout,

    #[nwg_control(flags: "VISIBLE")]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_70)]
    wb_frame: nwg::Frame,
    #[nwg_partial(parent: wb_frame)]
    #[nwg_shortcuts((visual, mass)  [A, D, NumpadSlash, NumpadTimes]: [OBPanelView::proc_nav_shortcut()])]
    #[nwg_events((mass) OnTextInput: [OBPanelView::check_data_entry(SELF,TARGET,HANDLE)])]
    product_wb: OBProductView,

    // #[nwg_partial(be: control)]
    #[nwg_partial_control]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_30)]
    product_range: OBRangesView,
}
impl OBPanelView {
    fn proc_nav_shortcut() {}
    fn check_data_entry(
        &self,
        field: &FixedNumEdit,
        handle: &ControlHandle,
    ) -> Result<(), std::num::ParseFloatError> {
        let val = field.parse()?;
        let ok_p = if handle == &self.product_wb.mass {
            self.product_range.mass.check(val)
        } else {
            false
        };

        if ok_p {
            field.set_border_color(None);
        } else {
            field.set_border_color(Some([0xff, 0, 0]));
        }

        Ok(())
    }

    pub fn click(&self) {
        self.product_range
            .mass
            // .set(&(vec![None, Some(5.1), None]).into());
            // .set(&vec![None, Some(5.1), None].into());
            .set(vec![Some(79.89), Some(88.01), Some(90.95)].into());
        // .set(&vec![Some(0.1), Some(5.1), Some(0.1115)].into());
        // .set(vec![None, Some(5.1), None].into());
        // .set(*vec![None, Some(5.1), None].into());
        self.product_range
            .sg
            // .set(&(vec![None, Some(5.1), None]).into());
            // .set(&vec![None, Some(5.1), None].into());
            .set(vec![Some(0.1), Some(5.1), Some(0.1115)].into());
    }
}

const PT_10: nwg::taffy::LengthPercentageAuto = nwg::taffy::LengthPercentageAuto::length(10.0);
const PT_35: nwg::taffy::LengthPercentageAuto = nwg::taffy::LengthPercentageAuto::length(35.0);
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
    #[nwg_control(text: "Test", border_color:Some([0xfe,0,0]))]
    #[nwg_layout_item(layout: frame_layout, margin: taffy::Rect {
        left: PT_10,
        right: PT_10,
        top: PT_35,
        bottom: PT_10,
    })]
    test: nwg::Label,

    #[nwg_control(text: "paddy", border_color:Some([0,0,0xfe]))]
    #[nwg_layout_item(layout: frame_layout)]
    paddy: nwg::Label,

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
