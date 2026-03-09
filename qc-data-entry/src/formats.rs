fn sig_fig(val: f32, precision: usize) -> String {
    format!("{:.*}", precision, val)
}

pub fn format_sg_mass(sg: f32) -> String {
    let precision: usize = if sg >= 1.0 { 3 } else { 4 };
    sig_fig(sg, precision)
    // sig_fig!(sg, precision)
}

pub fn format_density(density: f32) -> String {
    sig_fig(density, 3)
}

pub fn format_mass(mass: f32) -> String {
    sig_fig(mass, 2)
}
