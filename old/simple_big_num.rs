use rug::{Float};
use rug::ops::Pow;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    pub static DEFAULT_PRECISION: u32 = 100000;

    let mut sum: Float = Float::with_val(DEFAULT_PRECISION, 0.0);
    let max_elements: u32 = 5000;

    for n in 0..max_elements {
        let first_fraction_num: Float = Float::with_val(DEFAULT_PRECISION, Float::factorial(4 * n));
        let first_fraction_denom: Float = Float::with_val(DEFAULT_PRECISION,
                                                          Float::with_val(DEFAULT_PRECISION, 4).pow(n) * Float::with_val(DEFAULT_PRECISION, Float::factorial(n))).pow(4);

        let second_fraction_num: Float = Float::with_val(DEFAULT_PRECISION, 23690 * n + 1103);
        let second_fraction_denom: Float = Float::with_val(DEFAULT_PRECISION, Float::with_val(DEFAULT_PRECISION, 99).pow(4 * n));

        let first_fraction: Float = first_fraction_num / first_fraction_denom;
        let second_fraction: Float = second_fraction_num / second_fraction_denom;

        let sum_element: Float = first_fraction * second_fraction;
        sum += sum_element;
    }

    let first_const: Float = Float::with_val(DEFAULT_PRECISION, 8).sqrt() / Float::with_val(DEFAULT_PRECISION, 99).pow(2);

    let result: Float = Float::with_val(DEFAULT_PRECISION, 1) / (first_const * sum);

    println!("Pi is {:.5}", result);

    let correct_pi: Float = Float::with_val(DEFAULT_PRECISION, 22) / Float::with_val(DEFAULT_PRECISION, 7);
    let diff: Float = Float::with_val(DEFAULT_PRECISION, (result / correct_pi) * 100);
    println!("Diff is {:.5} %", diff.abs());
    println!("Time elapsed: {:.2?}", start_time.elapsed())
}
