fn exercise_3() {
    const ABSOLUTE_ZERO_C: f64 = -273.15;
    let mut temperature: f64 = 100.0;
    println!("STARTING TEMPERATURE: {}", temperature);
    temperature = -10.5;
    println!("The new temperature is: {}", temperature);
    println!("The absolute temperature: {}", ABSOLUTE_ZERO_C);
    let absolute_zero = temperature - ABSOLUTE_ZERO_C;
    println!("printing ABSOLUTE TEMPERATURE: {}", absolute_zero);
}

fn main() {
    exercise_3();
}