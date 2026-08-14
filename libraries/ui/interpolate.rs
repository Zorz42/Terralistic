/// Moves `value` a `1 / smooth_factor` fraction of the way to `target`, snapping onto it once
/// the two are within `epsilon`. Every fade and slide in the toolkit is one of these per
/// ready frame.
#[must_use]
pub fn approach(value: f32, target: f32, smooth_factor: f32, epsilon: f32) -> f32 {
    let stepped = value + (target - value) / smooth_factor;
    if (target - stepped).abs() <= epsilon {
        target
    } else {
        stepped
    }
}
