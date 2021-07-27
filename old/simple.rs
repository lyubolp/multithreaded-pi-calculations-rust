 use factorial::Factorial;

fn main() {
    let mut sum: f64 = 0.0;
    let max_elements: u32 = 3;

    for n in 0..max_elements {
        let first_fraction: f64 = (4*n).factorial() as f64 / ((4_u32.pow(n) * n.factorial()).pow(4)) as f64;
        let second_fraction: f64 = (23690 * n + 1103) as f64 / (99_f64.powf((4*n) as f64));

        let sum_element: f64 = first_fraction * second_fraction;

        sum += sum_element;
    }

    let first_const: f64 = 8_f64.sqrt() / (99_f64.powf(2_f64));

    let result: f64 = 1_f64 / (first_const * sum);

    println!("Pi is {}", result);
}
