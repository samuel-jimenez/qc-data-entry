extern crate native_windows_derive as nwd;

use std::cmp::Ordering;

use nwd::NwgPartial;
use nwg::{taffy::FlexDirection, KeyPress, ModifierKeys, NONAME};
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
    #[nwg_shortcuts((visual, viscosity, mass, string)  [A, D, NumpadSlash, NumpadTimes]: [FRPanelView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
    product_top: FRProductView,

    #[nwg_control(text: "Btm")]
    // #[nwg_layout_item(layout: frame_layout)]
    #[nwg_layout_item(layout: frame_layout, size:COL_40)]
    group_btm: nwg::GroupBox,
    #[nwg_partial(parent: group_btm)]
    #[nwg_shortcuts((visual, viscosity, mass, string)  [A, D, NumpadSlash, NumpadTimes]: [FRPanelView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
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

    fn proc_nav_shortcut(&self, combo: &nwg::KeyCombo, handle: &nwg::ControlHandle) {
        // match handle {
        //     self.visual.handle =>
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

// mabe ive struct memeber s to other?

#[derive_mass]
#[derive(Default, NwgPartial)]
// #[nwg_shortcuts((visual, viscosity, mass, string)  [W, S, NumpadMinus, NumpadPlus]: [FRProductView::proc_nav_shortcut(SELF,EVT,TARGET)])]
#[nwg_shortcuts((visual, viscosity, mass, string)  [W, S, NumpadMinus, NumpadPlus]: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
pub struct FRProductView {
    // Layout
    #[nwg_layout(flex_direction: FlexDirection::Column, padding:GROUP_PADDING)]
    frame_layout: nwg::FlexboxLayout,

    // Controls
    #[nwg_control(text: "Visual Inspection")]
    #[nwg_layout_item(layout: frame_layout,margin:VISUAL_MARGIN)]
    // #[nwg_shortcuts(W: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)], A: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)], S: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)], D: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
    // #[nwg_shortcuts(W: [FRProductView::test_shortcut(SELF,CTRL,HANDLE,EVT,EVT_DATA)])]
    // #[nwg_events(OnKeyPress: [FRProductView::test_event(SELF,CTRL,HANDLE,EVT)])]
    // #[nwg_shortcuts(W: [FRProductView::test_shortcut(SELF,CTRL,HANDLE,EVT)])]
    visual: nwg::CheckBox,

    // #[nwg_control(label: "Viscosity")]
    #[nwg_control(label: "Viscosity", number: true)]
    #[nwg_layout_item(layout: frame_layout)]
    // #[nwg_shortcuts(W: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)], A: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)], S: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)], D: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
    viscosity: nwg::LabeledEdit,

    // #[nwg_shortcuts(Shift+LBracket: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
    mass: nwg::LabeledEdit,

    // #[nwg_control(label: "String")]
    #[nwg_control(label: "String", number: true)]
    #[nwg_layout_item(layout: frame_layout)]
    // #[nwg_shortcuts(W: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)], A: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)], S: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)], D: [FRProductView::proc_nav_shortcut(SELF,EVT,HANDLE)])]
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
        // match handle {
        //     self.visual.handle =>
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

    //     NumpadTimes = MULTIPLY,
    // NumpadPlus = ADD,
    // Separator = SEPARATOR,
    // NumpadMinus = SUBTRACT,
    // Decimal = DECIMAL,
    // NumpadSlash = DIVIDE,

    fn test_shortcut(
        &self,
        u: &nwg::CheckBox,
        handle: &nwg::ControlHandle,
        combo: &nwg::KeyCombo,
        // mu: &nwg::CheckBox,
    ) {
        println!("woo");
    }
    fn test_event(
        &self,
        u: &nwg::CheckBox,
        handle: &nwg::ControlHandle,
        combo: &nwg::KeyCombo,
        // mu: &nwg::CheckBox,
    ) {
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
    string: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    sg: RangeView,

    #[nwg_partial]
    #[nwg_layout_item(layout: frame_layout)]
    density: RangeView,
}
