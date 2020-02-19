use regex::Regex;
use std::io;
use std::io::Write;

fn main() {
    let date_re =
        Regex::new(r"^(0[1-9]|1[012])[/](0[1-9]|[12][0-9]|3[01])[/](19|20)\d\d$").unwrap();
    let time_re = Regex::new(r"^(0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$").unwrap();

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

            let mut start_time = String::new();

            loop {
                print!("Enter the start time: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut start_time)
                    .expect("Failed to read line");

                if time_re.is_match(&start_time.trim()) {
                    break;
                } else {
                    println!("Invalid time. Required (24-hour time) format: hh:mm");
                    start_time = "".to_string();
                }
            }

            let mut end_date = String::new();

            loop {
                print!("Enter the end date: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut end_date)
                    .expect("Failed to read line");

                if date_re.is_match(&end_date.trim()) {
                    break;
                } else {
                    println!("Invalid date. Required format: mm/dd/yyyy");
                    end_date = "".to_string();
                }
            }

            let mut end_time = String::new();

            loop {
                print!("Enter the end time: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut end_time)
                    .expect("Failed to read line");

                if time_re.is_match(&end_time.trim()) {
                    break;
                } else {
                    println!("Invalid time. Required (24-hour time) format: hh:mm");
                    end_time = "".to_string();
                }
            }

            println!("Owner: {}", owner.trim());
            println!("Description: {}", description.trim());
            println!("Start date: {}", start_date.trim());
            println!("Start time: {}", start_time.trim());
            println!("End date: {}", end_date.trim());
            println!("End time: {}", end_time.trim());
        } else if input_option == 2 {
            println!("You entered {}", input_option);
            println!("This functionality isn't implemented yet");
        } else {
            println!("Goodbye");
            break;
        }
    }
}
