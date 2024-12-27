use std::io;

fn main() {
    println!("Please input temperature to convert");
        let mut temp = String::new();
    
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
    
        let temp: f32 =  temp
            .trim()
            .parse()
            .expect("not a number"); //match {
            //     Ok(num) => num,
            //     Err(_) => continue,
            // };
    
        println!("You want to convert: {}F", temp);

        let new_temp = (temp - 32.0)*(5.0/9.0);

        println!("which is equal to {}C", (new_temp*10.0).round()/10.0)
}
