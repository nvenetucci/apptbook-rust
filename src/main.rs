use std::io;

fn main() {
    loop {
        println!("\n1) Add an appointment");
        println!("2) Search for appointments");
        println!("3) Quit\n");

        println!("Enter an option number: ");

        let mut input_option = String::new();

        io::stdin()
            .read_line(&mut input_option)
            .expect("Failed to read line");

        let input_option: u32 = match input_option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option");
                continue;
            }
        };

        if input_option == 1 {
            let mut owner = String::new();

            println!("\nEnter the appointment's owner: ");
            io::stdin()
                .read_line(&mut owner)
                .expect("Failed to read line");

            println!("You entered {}", owner.trim());
        } else if input_option == 2 {
            println!("You entered {}", input_option);
            println!("This functionality isn't implemented yet");
        } else {
            println!("Goodbye");
            break;
        }
    }
}
