use super::POSITION_SIZE_EPSILON;

pub fn round_if_close(value: f32) -> f32 {
    let nearest = value.round();
    if (value - nearest).abs() < POSITION_SIZE_EPSILON {
        nearest
    } else {
        value
    }
}
