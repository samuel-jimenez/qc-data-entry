extern crate native_windows_derive as nwd;

use std::cmp::Ordering;

use nwd::NwgPartial;
use nwg::{taffy::FlexDirection, ControlHandle, KeyPress, LabeledEdit, ModifierKeys, NONAME};
use qc_data_entry::QCProduct;
use qc_data_entry_derive::derive_mass;

use crate::{
    constants::{COL_20, COL_30, COL_35, COL_40, GROUP_PADDING, VISUAL_MARGIN},
    number_edit_fixed::FixedNumEdit,
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
    #[nwg_shortcuts((visual, viscosity, mass, string)  [A, D, NumpadSlash, NumpadTimes]: [FRPanelView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
    #[nwg_events((mass) OnTextInput: [FRPanelView::check_data_entry(SELF,TARGET,HANDLE)],(viscosity, string) OnTextInput: [FRPanelView::check_data_entry_int(SELF,TARGET,HANDLE)])]
    product_top: FRProductView,

    #[nwg_control(text: "Btm")]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_40)]
    group_btm: nwg::GroupBox,
    #[nwg_partial(parent: group_btm)]
    #[nwg_shortcuts((visual, viscosity, mass, string)  [A, D, NumpadSlash, NumpadTimes]: [FRPanelView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
    #[nwg_events((mass) OnTextInput: [FRPanelView::check_data_entry(SELF,TARGET,HANDLE)],(viscosity, string) OnTextInput: [FRPanelView::check_data_entry_int(SELF,TARGET,HANDLE)])]
    product_btm: FRProductView,

    #[nwg_control]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_30)]
    group_range: nwg::Frame,
    #[nwg_partial(parent: group_range)]
    product_range: FRRangesView,
}

impl FRPanelView {
    pub(crate) fn update_product(&self, qc_product: &QCProduct) -> () {
        self.product_range.sg.set(&qc_product.sg);
        self.product_range
            .mass
            .set_with_map(&qc_product.sg, qc_data_entry::convert::mass_from_sg);
        self.product_range.density.set(&qc_product.density);
        self.product_range.string_test.set(&qc_product.string_test);
        self.product_range.viscosity.set(&qc_product.viscosity);
    }

    fn check_data_entry(
        &self,
        field: &FixedNumEdit,
        handle: &ControlHandle,
    ) -> Result<(), std::num::ParseFloatError> {
        let val = field.parse()?;
        let ok_p = if handle == &self.product_top.mass || handle == &self.product_btm.mass {
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
    fn check_data_entry_int(
        &self,
        field: &LabeledEdit,
        handle: &ControlHandle,
    ) -> Result<(), std::num::ParseFloatError> {
        let mut text = field.text();
        text.retain(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });
        let val = text.parse()?;
        let ok_p = if handle == &self.product_top.viscosity || handle == &self.product_btm.viscosity
        {
            self.product_range.viscosity.check(val)
        } else if handle == &self.product_top.string || handle == &self.product_btm.string {
            self.product_range.string_test.check(val)
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
    fn proc_nav_shortcut(&self, combo: &nwg::KeyCombo, handle: &nwg::ControlHandle) {
        match combo {
            nwg::KeyCombo {
                key: KeyPress::A, ..
            }
            | nwg::KeyCombo {
                key: KeyPress::NumpadSlash,
                ..
            } => {
                if handle == &self.product_top.visual || handle == &self.product_btm.visual {
                    self.product_top.visual.set_focus()
                } else if handle == &self.product_top.viscosity
                    || handle == &self.product_btm.viscosity
                {
                    self.product_top.viscosity.set_focus()
                } else if handle == &self.product_top.mass || handle == &self.product_btm.mass {
                    self.product_top.mass.set_focus()
                } else if handle == &self.product_top.string || handle == &self.product_btm.string {
                    self.product_top.string.set_focus()
                }
            }
            nwg::KeyCombo {
                key: KeyPress::D, ..
            }
            | nwg::KeyCombo {
                key: KeyPress::NumpadTimes,
                ..
            } => {
                if handle == &self.product_top.visual || handle == &self.product_btm.visual {
                    self.product_btm.visual.set_focus()
                } else if handle == &self.product_top.viscosity
                    || handle == &self.product_btm.viscosity
                {
                    self.product_btm.viscosity.set_focus()
                } else if handle == &self.product_top.mass || handle == &self.product_btm.mass {
                    self.product_btm.mass.set_focus()
                } else if handle == &self.product_top.string || handle == &self.product_btm.string {
                    self.product_btm.string.set_focus()
                }
            }
            _ => {}
        }
        println!("woo");
    }
}

#[derive_mass]
#[derive(Default, NwgPartial)]
#[nwg_shortcuts((visual, viscosity, mass, string)  [W, S, NumpadMinus, NumpadPlus]: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
pub struct FRProductView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout,margin:VISUAL_MARGIN)]
    visual: nwg::CheckBox,

    #[nwg_control(label: "Viscosity", number: true)]
    #[nwg_layout_item(layout: frame_layout)]
    viscosity: nwg::LabeledEdit,

    mass: FixedNumEdit,

    #[nwg_control(label: "String", number: true)]
    #[nwg_layout_item(layout: frame_layout)]
    string: nwg::LabeledEdit,
}

// filter u8
//place deimal at app spot

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
    fn next(&self, i: u32) -> String {
        self.viscosity.text()
    }
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
                } else if handle == &self.viscosity {
                    self.visual.set_focus()
                } else if handle == &self.mass {
                    self.viscosity.set_focus()
                } else if handle == &self.string {
                    self.mass.set_focus()
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
                    self.viscosity.set_focus()
                } else if handle == &self.viscosity {
                    self.mass.set_focus()
                } else if handle == &self.mass {
                    self.string.set_focus()
                } else if handle == &self.string {
                    self.string.set_focus()
                }
            }
            _ => {}
        }
        println!("woo");
    }

    fn test_shortcut(&self, u: &nwg::CheckBox, handle: &nwg::ControlHandle, combo: &nwg::KeyCombo) {
        println!("woo");
    }
    fn test_event(&self, u: &nwg::CheckBox, handle: &nwg::ControlHandle, combo: &nwg::KeyCombo) {
        println!("hoo");
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
    string_test: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    sg: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    density: RangeView,
}
