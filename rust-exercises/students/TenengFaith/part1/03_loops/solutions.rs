fn main(){
    exercise_1();
    exercise_2();
    exercise_3();
}

fn exercise_1(){
    for i in 1..31{
        if i%3==0 && i%5==0{
            println!("FizzBuzz");
        }
        else if i%3==0{
            println!("Fizz");
        }
        else if i%5==0{
            println!("Buzz");
        }
        else{
            println!("{}",1);
        }
    }
}

fn exercise_2(){
    let mut counter=2;
    let result= loop{
        counter+=1;
        if counter%7==0{
            let sqrt= (counter as f64).sqrt();
            if sqrt== sqrt.floor(){
            break counter;

            }
        }
    };

    println!("The first perfect square divisible by 7 is: {}", result);
}

fn exercise_3(){
    for i in 1..6{
        for j in 1..6{
            println!("{} x {} = {}",i,j,i*j);
        }
    }
}