pub mod pi_calculator {
    use rug::{Float};
    use rug::ops::Pow;
    use factorial::Factorial;
    use std::sync::mpsc::Sender;

    pub struct CalcData {

    }
    pub fn calculate_a_n_from_formula(n: u32, precision: u32) -> Float
    {
        let sqrt_8 = Float::with_val(precision, 8).sqrt();
        let first_const: Float = Float::with_val(precision, sqrt_8/9801f32);

        let mut second_const: Float = Float::with_val(precision, Float::factorial(4*n));
        let mut second_const_denom: Float = Float::with_val(precision, Float::factorial(n));
        second_const_denom *= Float::with_val(precision, Float::with_val(precision, 4).pow(n));
        second_const /= second_const_denom.pow(4);

        let mut third_const: Float = Float::with_val(precision, 1103 + 26390 * n);
        let mut third_const_denom: Float = Float::with_val(precision, 99);
        third_const_denom = third_const_denom.pow(4*n);
        third_const /= third_const_denom;


        first_const * second_const * third_const
    }
    pub fn calculate_a_n_from_previous(previous_a: &Float, previous_index: &u32, target_index: &u32, precision: &u32, sender: Sender<(Float, u32)>)
    {
        let mut final_const: Float = Float::with_val(*precision, 1);
        let result: Float;
        for current_index in *previous_index..*target_index
        {
            let mut first_part: Float = Float::with_val(*precision, 1);
            for i in 1..4{
                let temp = Float::with_val(*precision, (4*current_index) + i);
                first_part *= &temp;
            }
            let first_part_denom_first_part = Float::with_val(*precision, 4u32.pow(3));
            let first_part_denom_second_part:Float = Float::with_val(*precision,
                                                                     Float::with_val(*precision,current_index + 1).pow(3));

            let first_part_denom = first_part_denom_first_part * first_part_denom_second_part;
            first_part /= first_part_denom;

            let mut second_part: Float = Float::with_val(*precision, 1);
            second_part /= Float::with_val(*precision, 99u32.pow(4));

            let mut third_part: Float = Float::with_val(*precision, 26390);
            let mut third_part_denom: Float = Float::with_val(*precision, 105953739903i64);
            let mut third_part_denom_second_part = Float::with_val(*precision, 96059601);
            third_part_denom_second_part *= 26390;
            third_part_denom_second_part *= current_index;
            third_part_denom += third_part_denom_second_part;

            third_part /= third_part_denom;

            final_const *= first_part * (second_part + third_part);
        }
        result = final_const * previous_a;
        match sender.send((result, *target_index)){
            Err(_) => return,
            _ => {}
        }
    }
}