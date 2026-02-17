fn exercise_2() {
    let value = 2;
		let value = value * 10;
		let value = value + 5;
		let value = value.to_string();
		let value = format!("{}{}", value, " is the answer");

		println!("{}", value);
}
fn main(){
    exercise_2();
}