use std::io;

fn main() {
    println!("***Temperature Converter***");

    let mut choice = true;
    let mut usr_input = String::new();

    while choice {
        println!("1. F to C \n2. C to F");
        io::stdin()
            .read_line(&mut usr_input)
            .expect("Failed to read line");

        match usr_input.trim().parse::<u32>(){
            Ok(1) => {
                println!("You chose F to C");
                choice = true;
            }
            Ok(2) => {
                println!("You chose C to F");
                choice = false;
            }
            _ => {
                println!("Invalid choice, please enter 1 or 2.");
            }
        }

        println!("Please input temperature to convert");
        let mut temp = String::new();

        io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");
        
            let temp: f32 = match temp
                .trim()
                .parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

        if choice {              
            println!("You want to convert: {}F", temp);

            let new_temp = (temp - 32.0)*(5.0/9.0);

            println!("which is equal to {}C", (new_temp*10.0).round()/10.0)
        }
        else{
            println!("You want to convert: {}C", temp);

            let new_temp = (9.0/5.0 * temp)+32.0;

            println!("which is equal to {}C", (new_temp*10.0).round()/10.0)
        }

        usr_input.clear();
        println!("Would you like to continue?");
        println!("1. Yes \n2. No");
        io::stdin()
            .read_line(&mut usr_input)
            .expect("Failed to read line");

        match usr_input.trim().parse::<u32>(){
            Ok(1) => {
                choice = true;
                usr_input.clear();
            }
            Ok(2) => {
                println!("Good Bye");
                choice = false;
            }
            _ => {
                println!("Invalid choice, please enter 1 or 2.");
            }
        }
    }
}
