use regex::Regex;
use std::io;
use std::io::Write;

fn main() {
    let date_re =
        Regex::new(r"^(0[1-9]|1[012])[/](0[1-9]|[12][0-9]|3[01])[/](19|20)\d\d$").unwrap();

    loop {
        println!("\n1) Add an appointment");
        println!("2) Search for appointments");
        println!("3) Quit\n");

        print!("Enter an option number: ");
        io::stdout().flush().unwrap();

        let mut input_option = String::new();

        io::stdin()
            .read_line(&mut input_option)
            .expect("Failed to read line");

        let input_option: u32 = match input_option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option. Must be a valid number");
                continue;
            }
        };

        if input_option == 1 {
            let mut owner = String::new();

            print!("\nEnter the appointment's owner: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut owner)
                .expect("Failed to read line");

            let mut description = String::new();

            print!("Enter a description: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut description)
                .expect("Failed to read line");

            let mut start_date = String::new();

            loop {
                print!("Enter the start date: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut start_date)
                    .expect("Failed to read line");

                if date_re.is_match(&start_date.trim()) {
                    break;
                } else {
                    println!("Invalid date. Required format: mm/dd/yyyy");
                    start_date = "".to_string();
                }
            }

            println!("Owner: {}", owner.trim());
            println!("Description: {}", description.trim());
            println!("Start date: {}", start_date.trim());
        } else if input_option == 2 {
            println!("You entered {}", input_option);
            println!("This functionality isn't implemented yet");
        } else {
            println!("Goodbye");
            break;
        }
    }
}
