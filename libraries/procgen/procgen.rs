use noise::{NoiseFn, Perlin};
// rand 0.10 renamed RngCore to Rng; the old Rng is now RngExt
use rand::Rng;

/// How many octaves `turbulence` sums, each at half the amplitude and twice the frequency
/// of the last.
const TURBULENCE_OCTAVES: u32 = 3;

/// Fractal noise: several octaves of Perlin summed, each half the size of the one before, roughly
/// in `-1..1`.
///
/// One octave is smooth and featureless; a few give a coarse shape plus the roughness that makes
/// terrain look like terrain rather than a sine wave.
#[must_use]
pub fn turbulence(noise: &Perlin, x: f32, y: f32) -> f32 {
    let mut value = 0.0;
    let mut size = 1.0;

    for _ in 0..TURBULENCE_OCTAVES {
        value += noise.get([f64::from(x / size), f64::from(y / size)]) as f32 * size;
        size /= 2.0;
    }

    value / 2.0
}

/// Smooths an array by replacing each element with the mean of the `size` around it.
///
/// **The window is clipped at the ends, not wrapped or padded**, so an edge element is the mean of
/// however many neighbours it has - a world's edge should look like its neighbours rather than fade
/// to zero.
#[must_use]
pub fn convolve(array: &[f32], size: i32) -> Vec<f32> {
    let mut result = Vec::with_capacity(array.len());

    let mut sum = 0.0;
    let mut count = 0;
    for value in array.iter().take((size / 2) as usize) {
        sum += value;
        count += 1;
    }

    for i in 0..array.len() {
        if let Some(value) = array.get((i as i32 - size / 2) as usize) {
            sum -= value;
            count -= 1;
        }

        if let Some(value) = array.get((i as i32 + size / 2) as usize) {
            sum += value;
            count += 1;
        }

        result.push(sum / count as f32);
    }

    result
}

/// Picks one of `weighted` in proportion to its weight, or `None` if there is nothing to pick.
/// One step of a weighted random walk: the items are the edges out of where the walk is.
pub fn pick_weighted<'items, Item, Source: Rng>(weighted: &'items [(i32, Item)], rng: &mut Source) -> Option<&'items Item> {
    let total: i32 = weighted.iter().map(|(weight, _)| *weight).filter(|weight| *weight > 0).sum();
    if total <= 0 {
        return None;
    }

    let mut remaining = (rng.next_u32() % total as u32) as i32;
    for (weight, item) in weighted {
        if *weight <= 0 {
            continue;
        }
        remaining -= weight;
        if remaining < 0 {
            return Some(item);
        }
    }

    // Unreachable while the weights hold still, but a fallback beats an unwrap.
    weighted.last().map(|(_weight, item)| item)
}
