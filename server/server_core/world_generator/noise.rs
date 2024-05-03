use noise::{NoiseFn, Perlin};

pub(super) fn turbulence(noise: &Perlin, x: f32, y: f32) -> f32 {
    let mut value = 0.0;
    let mut size = 1.0;

    for _ in 0..3 {
        value += noise.get([(x / size) as f64, (y / size) as f64]) as f32 * size;
        size /= 2.0;
    }

    value / 2.0
}

pub(super) fn convolve(array: &[f32], size: i32) -> Vec<f32> {
    let mut result = Vec::new();

    let mut sum = 0.0;
    let mut count = 0;
    for i in array.iter().take((size / 2) as usize) {
        sum += i;
        count += 1;
    }

    for i in 0..array.len() {
        if let Some(val) = array.get((i as i32 - size / 2) as usize) {
            sum -= val;
            count -= 1;
        }

        if let Some(val) = array.get((i as i32 + size / 2) as usize) {
            sum += val;
            count += 1;
        }

        result.push(sum / count as f32);
    }

    result
}
