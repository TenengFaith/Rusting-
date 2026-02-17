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
fn main(){
    exercise_1() 
}