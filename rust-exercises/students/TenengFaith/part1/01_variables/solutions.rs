fn main(){
    exercise_1();
    exercise_2();
    exercise_3();
}


fn exercise_1() {
    //making x mutable so it's value can be altered
    let mut x = 10;
    x = 20;
    println!("x is {}", x);
// setting greetings as mutable so value can be altered
    let mut greeting = "hello";
    greeting = "goodbye";
    println!("{}", greeting);
//giving maxscore a type and setting it to upper case
    const MAX_SCORE:u32 = 100;
    println!("Max score is {}", MAX_SCORE);
}
//calling function in main function

fn exercise_2() {
    let value = 2;
		let value = value * 10;
		let value = value + 5;
		let value = value.to_string();
		let value = format!("{}{}", value, " is the answer");

		println!("{}", value);
}

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
