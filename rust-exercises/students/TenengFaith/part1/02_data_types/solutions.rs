fn main(){
    exercise_1();
    exercise_2();
    exercise_3();
}


fn exercise_1(){
    let value1: u8=255;
    let value2: i16 = -32000;
    let value3: f64 = 3.14159265358979;
    let value4: char= '🦀';
    let value5: bool= true;
    let value6: u32=1000000000;
    println!("{} is a u8",value1);
    println!("{} is an i16",value2);
    println!("{} is an f64",value3);
    println!("{} is a char",value4);
    println!("{} is a bool",value5);
    println!("{} is a u32",value6);
}

fn exercise_2(){
    let passport_tuple:(&str,u8,[char; 2],bool)=("John",12,['a','b'],true);
    let (name,age,country_code,is_valid)= passport_tuple;
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Country: {:?}",country_code);
    println!("Valid: {}",is_valid);
    println!("Age via dot notation: {}", passport_tuple.1);

}

fn exercise_3(){
    let  value1:u8=250;
    println!("u8 value : {}", value1);
    let value2:u16= value1 as u16 +1000;
    println!("u16 value after addition: {}", value2);
    // let value3:i32=201932;
    // let value3= value3 as i64;

    let frac:i32=7/2;
    println!("interger division 7/2= {}", frac);
    let frac1:f64= 7.0/2.0;
    println!("float division 7.0/2.0= {}", frac1);

    


}