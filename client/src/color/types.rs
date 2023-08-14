#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct HSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

// ik this is unreadable, i am not a mathematician
impl From<RGB> for HSV {
    fn from(rgb: RGB) -> Self {
        // https://www.had2know.org/technology/hsv-rgb-conversion-formula-calculator.html

        use std::cmp;

        let max = cmp::max(cmp::max(rgb.r, rgb.g), rgb.b);
        let min = cmp::min(cmp::min(rgb.r, rgb.g), rgb.b);

        let v = max as f32 / 255.;

        let s = if max > 0 {
            1. - (min as f32 / max as f32)
        } else {
            0.
        };

        let expression = f32::acos(
            ((rgb.r as f32) - (rgb.g as f32 * 0.5) - (rgb.b as f32 * 0.5))
                / f32::sqrt(
                    (rgb.r as f32).powf(2.) + (rgb.g as f32).powf(2.) + (rgb.b as f32).powf(2.)
                        - (rgb.r as f32 * rgb.g as f32)
                        - (rgb.r as f32 * rgb.b as f32)
                        - (rgb.g as f32 * rgb.b as f32),
                ),
        );

        let h = if rgb.g >= rgb.b {
            expression
        } else {
            360. - expression
        };

        HSV { h, s, v }
    }
}
