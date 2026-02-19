fn main(){
    exercise_1();
    exercise_2();
    exercise_3("red");
    exercise_3("yellow");
    exercise_3("green");
}

fn exercise_1(){
    let score1=95;
    let score2=72;
    let score3=45;
    if score1<100 && score1>90{
        println!("A- Excellent!");
    }
    else if score1>80{
        println!("B- Good");
    }
    else if score1>70{
        println!("c- Average");
    }
    else if score1>60{
        println!("D- Below Average");
    }
    else if score1>0{
        println!("F- Failing");
    }



     if score2<100 && score2>90{
        println!("A- Excellent!");
    }
    else if score2>80{
        println!("B- Good");
    }
    else if score2>70{
        println!("c- Average");
    }
    else if score2>60{
        println!("D- Below Average");
    }
    else if score2>0{
        println!("F- Failing");
    }



     if score3<100 && score3>90{
        println!("A- Excellent!");
    }
    else if score3>80{
        println!("B- Good");
    }
    else if score3>70{
        println!("c- Average");
    }
    else if score3>60{
        println!("D- Below Average");
    }
    else if score3>0{
        println!("F- Failing");
    }

}

fn exercise_2(){
    let n=17;
    if n%2==0{
        println!("{} is even",n);
        if n>100{
            println!("{} is large",n);
        }
        else if n>10{
            println!("{} is medium",n);
        }
        else{
            println!("{} is small",n);
        }
    }
    else{
        println!("{} is odd",n);
        if n>100{
            println!("{} is large",n);
        }
        else if n>10{
            println!("{} is medium",n);
        }
        else{
            println!("{} is small",n);
        }

    }
}

fn exercise_3(light: &str){
    let meaning: &str;
    let duration: i32;
    let safe_to_cross: &str;
    
    let display_name = if light == "red" {
        "Red"
    } else if light == "yellow" {
        "Yellow"
    } else {
        "Green"
    };

    if light == "red" {
        meaning = "Stop";
        duration = 30;
        safe_to_cross = "No";
    } else if light == "yellow" {
        meaning = "Caution";
        duration = 5;
        safe_to_cross = "No";
    } else {
        meaning = "Go";
        duration = 25;
        safe_to_cross = "Yes";
    }

    println!("{display_name} light: {meaning} | Duration: {duration}s | Safe to cross: {safe_to_cross}");
}
