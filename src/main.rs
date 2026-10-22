use std::io;
fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let first_line: i32 = match input1.trim().parse() {
        Ok(n) if n >0 => n,
        _ => {
           print!("you must enter a number");
           return;
        }
    };
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let second_line = match input2.trim().parse(){
        Ok(n) if n > 0 => n,
        _ =>{
            print!("you must write a number");
            return;
        }
    };
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Faild to read line");
    let third_line = match input3.trim().parse(){
        Ok(n) if n > 0 => n,
        _ =>{
            print!("you must write a number");
            return;
        }
    };

    if first_line + second_line > third_line && first_line + third_line > second_line && second_line + third_line > first_line {
        if first_line == second_line && second_line == third_line {
            println!("Equilateral");
        }
        else if first_line == second_line || first_line == third_line || second_line == third_line {
            println!("Isosceles");
        }
        else if (first_line*first_line) + (second_line*second_line) == (third_line*third_line) || (first_line*first_line) + (third_line*third_line) == (second_line*second_line) || (second_line*second_line) + (third_line*third_line) == (first_line*first_line){
            println!("Right-angled");
        }
        else {
            println!("Triangle");
        }

    }
    else{
        println!("Not a Triangle");
    }




}
