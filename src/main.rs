use std::io;

fn main() {
    println!("Welcome to the Banking System!");
    let mut quit: bool = false;
    let mut balance: f64 = 0.0;
    
    while !quit {
        println!("Choose an option:");
        println!("1- Withdraw");
        println!("2- Deposit");
        println!("3- Balance");
        println!("4- Quit");
        
        //let possible_options: (u8, u8, u8, u8) = (1, 2, 3, 4);
        let mut chosen_option: String = String::new();

        io::stdin()
            .read_line(&mut chosen_option)
            .expect("Failed to read line");
        
        let chosen_option: u8 = match chosen_option.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid option.");
                continue;
            },
        };

        match chosen_option{
            1 => {
                let mut amount: String = String::new();
                println!("How much do you want to withdraw?");
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line.");
                let amount: f64 = convert_to_f64(amount);
                if amount <= 0.0 || amount > balance{
                    println!("Not a valid amount.");
                    continue;
                }
                balance -= amount;
            },
            2 => {
                let mut amount: String = String::new();
                println!("How much do you want to deposit?");
                io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line.");
                let amount = convert_to_f64(amount);
                if amount <= 0.0 {
                    println!("Not a valid amount.");
                    continue;
                }
                balance += amount;
            },
            3 =>{
                println!("Your balance is {balance}");
            },
            4 => {
                quit = true;
                println!("Quitting...");
            },
            _ => {
                println!("Not a valid option");
            }
        }
    }
}

fn convert_to_f64(value: String) -> f64{
    match value.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid option.");
            return 0.0;
        }
    }
}