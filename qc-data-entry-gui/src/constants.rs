use nwg::taffy::{
    style_helpers::{auto, zero},
    Dimension as D, LengthPercentage, LengthPercentageAuto, Rect, Size,
};

const VISUAL_MARGIN_SPACING: LengthPercentageAuto = LengthPercentageAuto::length(10.0);
pub(crate) const VISUAL_MARGIN: Rect<LengthPercentageAuto> = Rect {
    left: VISUAL_MARGIN_SPACING,
    right: zero(),
    top: zero(),
    bottom: zero(),
};

const PT_10: LengthPercentage = LengthPercentage::length(10.0);
// this obviously will be font dependent.
// TODO add a builder flag to auto-size client area.
const PT_35: LengthPercentage = LengthPercentage::length(35.0);

pub(crate) const GROUP_PADDING: Rect<LengthPercentage> = Rect {
    left: PT_10,
    right: PT_10,
    top: PT_35,
    bottom: PT_10,
};

const PC_20: D = D::percent(0.2);
const PC_40: D = D::percent(0.4);
const PC_80: D = D::percent(0.8);

pub const COL_20: Size<D> = Size {
    width: PC_20,
    height: auto(),
};

pub const COL_40: Size<D> = Size {
    width: PC_40,
    height: auto(),
};

pub const COL_80: Size<D> = Size {
    width: PC_80,
    height: auto(),
};

pub const SAMPLE_VOLUME: f32 = 83.2;
pub const LB_PER_GAL: f32 = 8.34; // g/mL
