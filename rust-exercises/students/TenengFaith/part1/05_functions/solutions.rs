fn main(){
    exercise_1();
}

fn is_even(n: i32) -> bool {
    // This expression evaluates to true or false.
    n % 2 == 0
}

fn classify_temperature(temp: f64) -> &'static str {
    // In Rust, 'if' is an expression, not just a control flow statement.
    // The entire block evaluates to the string slice in the matching branch.
    if temp < 0.0 {
        "freezing"
    } else if temp < 15.0 {
        "cold"
    } else if temp < 25.0 {
        "warm"
    } else {
        "hot"
    }
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    // Returns the result of the division/multiplication expression.
    weight_kg / (height_m * height_m)
}

fn describe_bmi(bmi: f64) -> &'static str {
    // Returns a string based on the evaluation of the BMI range.
    if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Normal"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obese"
    }
}

fn exercise_1() {
    let test_n = 42;
    let test_temp = 12.5;
    let weight = 75.0;
    let height = 1.8;

    println!("Is {} even? {}", test_n, is_even(test_n));

    println!("Temperature {} is: {}", test_temp, classify_temperature(test_temp));

    let bmi = calculate_bmi(weight, height);
    let description = describe_bmi(bmi);
    
    println!("BMI for weight {}kg and height {}m is: {:.2}", weight, height, bmi);
    println!("BMI Category: {}", description);
}