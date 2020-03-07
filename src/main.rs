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

    let mut appts = String::new();

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

    if appts.is_empty() {
        appts.push_str("{}");
    }

    let mut apptbook: HashMap<String, Vec<Appointment>> = serde_json::from_str(&appts).unwrap();

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

            print!("Enter the description: ");
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

            owner = owner.trim().to_string();
            let to_sort = owner.clone();

            description = description.trim().to_string();
            let start_date_time = format!("{} {}", start_date.trim(), start_time.trim());
            let end_date_time = format!("{} {}", end_date.trim(), end_time.trim());

            let sdt = NaiveDateTime::parse_from_str(&start_date_time, "%m/%d/%Y %H:%M").unwrap();
            let edt = NaiveDateTime::parse_from_str(&end_date_time, "%m/%d/%Y %H:%M").unwrap();

            let appt = Appointment {
                description,
                start_date_time: sdt,
                end_date_time: edt,
            };

            apptbook.entry(owner).or_insert_with(Vec::new).push(appt);

            let owners_vec = apptbook.get_mut(&to_sort).unwrap();

            owners_vec.sort_by(|a, b| {
                a.start_date_time
                    .cmp(&b.start_date_time)
                    .then(a.end_date_time.cmp(&b.end_date_time))
                    .then(a.description.cmp(&b.description))
            });

            println!("\nAppointment added successfully");
        } else if input_option == 2 {
            if apptbook.is_empty() {
                println!("Appointment book is empty. Try adding an appointment")
            } else {
                println!("\nAvailable appointment owners:");
                for owner in apptbook.keys() {
                    println!("* {}", owner);
                }

                let mut owner_to_search = String::new();

                print!("\nShow appointments for: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut owner_to_search)
                    .expect("Failed to read line");

                if let Some(appts) = apptbook.get(owner_to_search.trim()) {
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
        } else {
            println!("\nGoodbye\n");
            break;
        }
    }

    let serialized = serde_json::to_string(&apptbook).unwrap();

    fs::write("apptbook.txt", serialized).expect("Failed to write file");
}
