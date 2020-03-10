use chrono::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Appointment {
    description: String,
    start_date_time: NaiveDateTime,
    end_date_time: NaiveDateTime,
}

fn main() {
    let date_re =
        Regex::new(r"^(0[1-9]|1[012])[/](0[1-9]|[12][0-9]|3[01])[/](19|20)\d\d$").unwrap();
    let time_re = Regex::new(r"^(0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$").unwrap();

    // String to hold read-in HashMap from storage file
    let mut appts = String::new();

    // Attempt to read-in storage file. If file doesn't exist, create one
    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("apptbook.txt")
    {
        Ok(ref mut file) => {
            file.read_to_string(&mut appts)
                .expect("Failed to read file");
        }
        Err(err) => {
            panic!("Failed to open file: {}", err);
        }
    }

    // If String empty, allocate with "{}" to represent an empty HashMap
    if appts.is_empty() {
        appts.push_str("{}");
    }

    let mut apptbook: HashMap<String, Vec<Appointment>> = serde_json::from_str(&appts).unwrap();

    loop {
        println!("\n1) Add appointment");
        println!("2) View appointments");
        println!("3) Delete appointments");
        println!("4) Quit\n");

        print!("Enter an option number: ");
        io::stdout().flush().unwrap();

        let mut input_option = String::new();

        io::stdin()
            .read_line(&mut input_option)
            .expect("Failed to read line");

        // Check if the user entered a number. If NaN, prompt again
        let input_option: u32 = match input_option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option. Must be a valid option number");
                continue;
            }
        };

        if input_option == 1 {
            // Add an appointment option
            let mut owner = String::new();
            let mut description = String::new();
            let mut start_date = String::new();
            let mut start_time = String::new();
            let mut end_date = String::new();
            let mut end_time = String::new();

            // Prompt for owner
            print!("\nEnter the appointment's owner: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut owner)
                .expect("Failed to read line");

            // Prompt for description
            print!("Enter the description: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut description)
                .expect("Failed to read line");

            // Prompt for start date. If invalid input, try again
            loop {
                print!("Enter the start date: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut start_date)
                    .expect("Failed to read line");

                // Check that start_date matches the date regex
                if date_re.is_match(&start_date.trim()) {
                    break;
                } else {
                    println!("Invalid date. Required format: mm/dd/yyyy");
                    start_date = "".to_string();
                }
            }

            // Prompt for start time. If invalid input, try again
            loop {
                print!("Enter the start time: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut start_time)
                    .expect("Failed to read line");

                // Check that start_time matches the time regex
                if time_re.is_match(&start_time.trim()) {
                    break;
                } else {
                    println!("Invalid time. Required (24-hour clock) format: hh:mm");
                    start_time = "".to_string();
                }
            }

            // Prompt for end date. If invalid input, try again
            loop {
                print!("Enter the end date: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut end_date)
                    .expect("Failed to read line");

                // Check that end_date matches the date regex
                if date_re.is_match(&end_date.trim()) {
                    // setup start_date and end_date for validation
                    let formatted_sd = format!("{} {}", start_date.trim(), "00:00");
                    let formatted_ed = format!("{} {}", end_date.trim(), "00:00");
                    let sd =
                        NaiveDateTime::parse_from_str(&formatted_sd, "%m/%d/%Y %H:%M").unwrap();
                    let ed =
                        NaiveDateTime::parse_from_str(&formatted_ed, "%m/%d/%Y %H:%M").unwrap();

                    // make sure end_date doesn't occur before start_date
                    if ed < sd {
                        println!("Invalid date. End date cannot occur before start date");
                        end_date = "".to_string();
                    } else {
                        // the end_date is approved
                        break;
                    }
                } else {
                    println!("Invalid date. Required format: mm/dd/yyyy");
                    end_date = "".to_string();
                }
            }

            // Prompt for end time. If invalid input, try again
            loop {
                print!("Enter the end time: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut end_time)
                    .expect("Failed to read line");

                // Check that end_time matches the time regex
                if time_re.is_match(&end_time.trim()) {
                    // Setup start_time and end_time for validation
                    let formatted_st = format!("{} {}", start_date.trim(), start_time.trim());
                    let formatted_et = format!("{} {}", end_date.trim(), end_time.trim());
                    let st =
                        NaiveDateTime::parse_from_str(&formatted_st, "%m/%d/%Y %H:%M").unwrap();
                    let et =
                        NaiveDateTime::parse_from_str(&formatted_et, "%m/%d/%Y %H:%M").unwrap();

                    // Make sure end_time doesn't occur before start_time
                    if et < st {
                        println!("Invalid time. End time cannot occur before start time");
                        end_time = "".to_string();
                    } else {
                        // The end_time is approved
                        break;
                    }
                } else {
                    println!("Invalid time. Required (24-hour clock) format: hh:mm");
                    end_time = "".to_string();
                }
            }

            owner = owner.trim().to_string();
            description = description.trim().to_string();

            // Format start_date/time and end_date/time into one String
            let start_date_time = format!("{} {}", start_date.trim(), start_time.trim());
            let end_date_time = format!("{} {}", end_date.trim(), end_time.trim());

            // Parse start_date_time and end_date_time into NaiveDateTimes
            let sdt = NaiveDateTime::parse_from_str(&start_date_time, "%m/%d/%Y %H:%M").unwrap();
            let edt = NaiveDateTime::parse_from_str(&end_date_time, "%m/%d/%Y %H:%M").unwrap();

            let appt = Appointment {
                description,
                start_date_time: sdt,
                end_date_time: edt,
            };

            // Clone owner to use again after the move
            let to_sort = owner.clone();

            // Push owner's Appointment to their Vec in the HashMap. If the owner doesn't already
            // exist within the HashMap, create for them a new Vec, then push the Appointment
            apptbook.entry(owner).or_insert_with(Vec::new).push(appt);

            let owners_vec = apptbook.get_mut(&to_sort).unwrap();

            // Sort owner's Appointment Vec by start_date_time. If start_date_times are equal, sort
            // by end_date_time. If end_date_times are equal, sort by description (alphabetically)
            owners_vec.sort_by(|a, b| {
                a.start_date_time
                    .cmp(&b.start_date_time)
                    .then(a.end_date_time.cmp(&b.end_date_time))
                    .then(a.description.cmp(&b.description))
            });

            println!("\nAppointment added successfully");
        } else if input_option == 2 {
            // View appointments option
            if apptbook.is_empty() {
                println!("Appointment book is empty. Try adding an appointment")
            } else {
                println!("\nAvailable appointment owners:");
                for owner in apptbook.keys() {
                    println!("* {}", owner);
                }

                let mut owner = String::new();

                print!("\nView appointments for: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut owner)
                    .expect("Failed to read line");

                // If the owner exists within the HashMap, pretty print their Appointments
                if let Some(appts) = apptbook.get(owner.trim()) {
                    for appt in appts {
                        let formatted_sdt =
                            appt.start_date_time.format("%m/%d/%Y %H:%M").to_string();

                        let formatted_edt =
                            if appt.start_date_time.date() == appt.end_date_time.date() {
                                appt.end_date_time.format("%H:%M").to_string()
                            } else {
                                appt.end_date_time.format("%m/%d/%Y %H:%M").to_string()
                            };

                        println!("\n> {}", appt.description);
                        println!("| {} to {}", formatted_sdt, formatted_edt);
                        println!(
                            "| Duration: {} minutes",
                            (appt.end_date_time - appt.start_date_time).num_minutes()
                        );
                    }
                } else {
                    println!("There are currently no appointments for that owner");
                }
            }
        } else if input_option == 3 {
            // Delete appointments option
            if apptbook.is_empty() {
                println!("Appointment book is empty. Try adding an appointment")
            } else {
                println!("\nAvailable appointment owners:");
                for owner in apptbook.keys() {
                    println!("* {}", owner);
                }

                let mut owner = String::new();

                print!("\nDelete appointments for: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut owner)
                    .expect("Failed to read line");

                // If the owner exists within the HashMap, ...
                if apptbook.contains_key(owner.trim()) {
                    loop {
                        println!("\n1) Delete all");
                        println!("2) Delete selected");
                        println!("3) Cancel\n");

                        print!("Enter an option number: ");
                        io::stdout().flush().unwrap();

                        let mut delete_option = String::new();

                        io::stdin()
                            .read_line(&mut delete_option)
                            .expect("Failed to read line");

                        match delete_option.trim() {
                            "1" => delete_all(owner.trim(), &mut apptbook),
                            "2" => delete_selected(owner.trim(), &mut apptbook),
                            "3" => (),
                            _ => {
                                println!("Invalid option. Must be a valid option number");
                                continue;
                            }
                        }

                        break;
                    }
                } else {
                    println!("There are currently no appointments for that owner");
                }
            }
        } else {
            println!("\nGoodbye\n");
            break;
        }
    }

    let serialized = serde_json::to_string(&apptbook).unwrap();

    // Save HashMap to storage file
    fs::write("apptbook.txt", serialized).expect("Failed to write file");
}

fn delete_all(owner: &str, apptbook: &mut HashMap<String, Vec<Appointment>>) {
    println!();

    loop {
        print!("Delete all appointments for {}? (y or n): ", owner);
        io::stdout().flush().unwrap();

        let mut confirm = String::new();
        io::stdin()
            .read_line(&mut confirm)
            .expect("Failed to read line");

        match confirm.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                apptbook.remove(owner);
                println!("\nAppointments deleted successfully");
            }
            "n" | "no" => println!("\nCanceled deleting appointments"),
            _ => continue,
        }

        break;
    }
}

fn delete_selected(owner: &str, apptbook: &mut HashMap<String, Vec<Appointment>>) {
    let appts = apptbook.get_mut(owner).unwrap();
    let mut count = 0;

    // Display all owner's appointments
    for appt in &*appts {
        count += 1;

        let formatted_sdt = appt.start_date_time.format("%m/%d/%Y %H:%M").to_string();

        let formatted_edt = if appt.start_date_time.date() == appt.end_date_time.date() {
            appt.end_date_time.format("%H:%M").to_string()
        } else {
            appt.end_date_time.format("%m/%d/%Y %H:%M").to_string()
        };

        println!(
            "\n<{}> {} | {} to {}",
            count, appt.description, formatted_sdt, formatted_edt
        );
    }

    loop {
        // Prompt user for which appointment to delete
        print!("\nEnter the number of the appointment to delete: ");
        io::stdout().flush().unwrap();

        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        // check if user input is a number
        let num: usize = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Must be a valid appointment number");
                continue;
            }
        };

        if num > count || num < 1 {
            println!("Invalid number. Must be a valid appointment number");
            continue;
        }

        println!();

        loop {
            // Confirm deletion of selected appointment
            print!("Delete appointment <{}> for {}? (y or n): ", num, owner);
            io::stdout().flush().unwrap();

            let mut confirm = String::new();
            io::stdin()
                .read_line(&mut confirm)
                .expect("Failed to read line");

            match confirm.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    // If only one appointment in Vec, delete owner from HashMap
                    if count == 1 {
                        apptbook.remove(owner);
                    } else {
                        appts.remove(num - 1);
                    }
                    println!("\nAppointment deleted successfully");
                }
                "n" | "no" => println!("\nCanceled deleting appointment"),
                _ => continue,
            }

            break;
        }

        break;
    }
}
