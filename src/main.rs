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
    // date regex received from https://stackoverflow.com/questions/15491894/regex-to-validate-date-format-dd-mm-yyyy
    let date_re =
        Regex::new(r"^(((0[13-9]|1[012])[/](0[1-9]|[12][0-9]|30)|(0[13578]|1[02])[/]31|02[/](0[1-9]|1[0-9]|2[0-8]))[/][0-9]{4}|02[/]29[/]([0-9]{2}(([2468][048]|[02468][48])|[13579][26])|([13579][26]|[02468][048]|0[0-9]|1[0-6])00))$").unwrap();

    // time regex received from https://stackoverflow.com/questions/7536755/regular-expression-for-matching-hhmm-time-format/7536768
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
                    println!("Invalid date. Required format: mm/dd/yyyy\n");
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
                    println!("Invalid time. Required (24-hour clock) format: hh:mm\n");
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
                        println!("Invalid date. End date cannot occur before start date\n");
                        end_date = "".to_string();
                    } else {
                        // the end_date is approved
                        break;
                    }
                } else {
                    println!("Invalid date. Required format: mm/dd/yyyy\n");
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
                        println!("Invalid time. End time cannot occur before start time\n");
                        end_time = "".to_string();
                    } else {
                        // The end_time is approved
                        break;
                    }
                } else {
                    println!("Invalid time. Required (24-hour clock) format: hh:mm\n");
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
        } else if input_option == 4 {
            // Quit option
            println!("\nGoodbye\n");
            break;
        } else {
            // Invalid option
            println!("Invalid option. Must be a valid option number");
        }
    }

    let serialized = serde_json::to_string(&apptbook).unwrap();

    // Save HashMap to storage file
    fs::write("apptbook.txt", serialized).expect("Failed to write file");
}

fn delete_all(owner: &str, apptbook: &mut HashMap<String, Vec<Appointment>>) {
    println!();

    loop {
        // Confirm deletion of all appointments
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

//===============================================================================================
//===== Unit Tests

#[test]
fn regex_date() {
    let date_re =
        Regex::new(r"^(((0[13-9]|1[012])[/](0[1-9]|[12][0-9]|30)|(0[13578]|1[02])[/]31|02[/](0[1-9]|1[0-9]|2[0-8]))[/][0-9]{4}|02[/]29[/]([0-9]{2}(([2468][048]|[02468][48])|[13579][26])|([13579][26]|[02468][048]|0[0-9]|1[0-6])00))$").unwrap();

    assert_eq!(date_re.is_match("02/02/2020"), true);
    assert_eq!(date_re.is_match("02/29/2020"), true);
    assert_eq!(date_re.is_match("02/30/2020"), false);
    assert_eq!(date_re.is_match("02/31/2020"), false);
    assert_eq!(date_re.is_match("04/30/2020"), true);
    assert_eq!(date_re.is_match("04/31/2020"), false);
}

#[test]
fn regex_time() {
    let time_re = Regex::new(r"^(0[0-9]|1[0-9]|2[0-3]):[0-5][0-9]$").unwrap();

    assert_eq!(time_re.is_match("00:00"), true);
    assert_eq!(time_re.is_match("23:59"), true);
    assert_eq!(time_re.is_match("24:00"), false);
    assert_eq!(time_re.is_match("24:30"), false);
    assert_eq!(time_re.is_match("12:60"), false);
    assert_eq!(time_re.is_match("12:59"), true);
    assert_eq!(time_re.is_match("00:32"), true);
}

#[test]
fn serialize_deserialize_apptbook() {
    let start_date_time = "02/02/2020 11:30";
    let end_date_time = "02/02/2020 12:15";

    let appt = Appointment {
        description: "Have lunch with Lisa".to_string(),
        start_date_time: NaiveDateTime::parse_from_str(&start_date_time, "%m/%d/%Y %H:%M").unwrap(),
        end_date_time: NaiveDateTime::parse_from_str(&end_date_time, "%m/%d/%Y %H:%M").unwrap(),
    };

    let mut apptbook: HashMap<String, Vec<Appointment>> = HashMap::new();

    apptbook
        .entry("Tom".to_string())
        .or_insert_with(Vec::new)
        .push(appt);

    let serialized = serde_json::to_string(&apptbook).unwrap();
    let deserialized: HashMap<String, Vec<Appointment>> =
        serde_json::from_str(&serialized).unwrap();

    let toms_vec = deserialized.get("Tom").unwrap();

    assert_eq!(toms_vec[0].description, "Have lunch with Lisa");
    assert_eq!(
        toms_vec[0].start_date_time,
        NaiveDateTime::parse_from_str("02/02/2020 11:30", "%m/%d/%Y %H:%M").unwrap()
    );
    assert_eq!(
        toms_vec[0].end_date_time,
        NaiveDateTime::parse_from_str("02/02/2020 12:15", "%m/%d/%Y %H:%M").unwrap()
    );
}

#[test]
fn can_sort_date_time() {
    let dt1 = NaiveDateTime::parse_from_str("02/02/2020 11:30", "%m/%d/%Y %H:%M").unwrap();
    let dt2 = NaiveDateTime::parse_from_str("02/02/2020 11:00", "%m/%d/%Y %H:%M").unwrap();
    let dt3 = NaiveDateTime::parse_from_str("02/02/2020 11:15", "%m/%d/%Y %H:%M").unwrap();
    let dt4 = NaiveDateTime::parse_from_str("02/02/1999 12:30", "%m/%d/%Y %H:%M").unwrap();

    let mut vec: Vec<NaiveDateTime> = Vec::new();

    vec.push(dt1);
    vec.push(dt2);
    vec.push(dt3);
    vec.push(dt4);

    vec.sort();

    assert_eq!(vec, vec![dt4, dt2, dt3, dt1]);
}

#[test]
fn sort_by_start_date_time() {
    let sdt1 = NaiveDateTime::parse_from_str("02/15/2020 14:30", "%m/%d/%Y %H:%M").unwrap();
    let edt1 = NaiveDateTime::parse_from_str("02/15/2020 15:00", "%m/%d/%Y %H:%M").unwrap();
    let sdt2 = NaiveDateTime::parse_from_str("02/16/2020 08:15", "%m/%d/%Y %H:%M").unwrap();
    let edt2 = NaiveDateTime::parse_from_str("02/16/2020 09:00", "%m/%d/%Y %H:%M").unwrap();
    let sdt3 = NaiveDateTime::parse_from_str("02/14/2020 19:30", "%m/%d/%Y %H:%M").unwrap();
    let edt3 = NaiveDateTime::parse_from_str("02/14/2020 19:45", "%m/%d/%Y %H:%M").unwrap();

    let appt1 = Appointment {
        description: "Do homework".to_string(),
        start_date_time: sdt1,
        end_date_time: edt1,
    };
    let appt2 = Appointment {
        description: "Do more homework".to_string(),
        start_date_time: sdt2,
        end_date_time: edt2,
    };
    let appt3 = Appointment {
        description: "Do even more homework".to_string(),
        start_date_time: sdt3,
        end_date_time: edt3,
    };

    let mut vec: Vec<Appointment> = Vec::new();
    vec.push(appt1);
    vec.push(appt2);
    vec.push(appt3);

    vec.sort_by(|a, b| {
        a.start_date_time
            .cmp(&b.start_date_time)
            .then(a.end_date_time.cmp(&b.end_date_time))
            .then(a.description.cmp(&b.description))
    });

    assert_eq!(vec[0].description, "Do even more homework");
    assert_eq!(vec[1].description, "Do homework");
    assert_eq!(vec[2].description, "Do more homework");
}

#[test]
fn sort_by_end_date_time() {
    let sdt1 = NaiveDateTime::parse_from_str("02/07/2020 08:30", "%m/%d/%Y %H:%M").unwrap();
    let edt1 = NaiveDateTime::parse_from_str("02/16/2020 15:00", "%m/%d/%Y %H:%M").unwrap();
    let sdt2 = NaiveDateTime::parse_from_str("02/07/2020 08:30", "%m/%d/%Y %H:%M").unwrap();
    let edt2 = NaiveDateTime::parse_from_str("02/15/2020 15:00", "%m/%d/%Y %H:%M").unwrap();
    let sdt3 = NaiveDateTime::parse_from_str("02/07/2020 08:30", "%m/%d/%Y %H:%M").unwrap();
    let edt3 = NaiveDateTime::parse_from_str("02/17/2020 15:00", "%m/%d/%Y %H:%M").unwrap();

    let appt1 = Appointment {
        description: "Do homework".to_string(),
        start_date_time: sdt1,
        end_date_time: edt1,
    };
    let appt2 = Appointment {
        description: "Do more homework".to_string(),
        start_date_time: sdt2,
        end_date_time: edt2,
    };
    let appt3 = Appointment {
        description: "Do even more homework".to_string(),
        start_date_time: sdt3,
        end_date_time: edt3,
    };

    let mut vec: Vec<Appointment> = Vec::new();
    vec.push(appt1);
    vec.push(appt2);
    vec.push(appt3);

    vec.sort_by(|a, b| {
        a.start_date_time
            .cmp(&b.start_date_time)
            .then(a.end_date_time.cmp(&b.end_date_time))
            .then(a.description.cmp(&b.description))
    });

    assert_eq!(vec[0].description, "Do more homework");
    assert_eq!(vec[1].description, "Do homework");
    assert_eq!(vec[2].description, "Do even more homework");
}

#[test]
fn sort_by_description() {
    let sdt1 = NaiveDateTime::parse_from_str("02/25/2020 06:00", "%m/%d/%Y %H:%M").unwrap();
    let edt1 = NaiveDateTime::parse_from_str("02/25/2020 07:00", "%m/%d/%Y %H:%M").unwrap();
    let sdt2 = NaiveDateTime::parse_from_str("02/25/2020 06:00", "%m/%d/%Y %H:%M").unwrap();
    let edt2 = NaiveDateTime::parse_from_str("02/25/2020 07:00", "%m/%d/%Y %H:%M").unwrap();
    let sdt3 = NaiveDateTime::parse_from_str("02/25/2020 06:00", "%m/%d/%Y %H:%M").unwrap();
    let edt3 = NaiveDateTime::parse_from_str("02/25/2020 07:00", "%m/%d/%Y %H:%M").unwrap();

    let appt1 = Appointment {
        description: "Do homework".to_string(),
        start_date_time: sdt1,
        end_date_time: edt1,
    };
    let appt2 = Appointment {
        description: "Do more homework".to_string(),
        start_date_time: sdt2,
        end_date_time: edt2,
    };
    let appt3 = Appointment {
        description: "Do even more homework".to_string(),
        start_date_time: sdt3,
        end_date_time: edt3,
    };

    let mut vec: Vec<Appointment> = Vec::new();
    vec.push(appt1);
    vec.push(appt2);
    vec.push(appt3);

    vec.sort_by(|a, b| {
        a.start_date_time
            .cmp(&b.start_date_time)
            .then(a.end_date_time.cmp(&b.end_date_time))
            .then(a.description.cmp(&b.description))
    });

    assert_eq!(vec[0].description, "Do even more homework");
    assert_eq!(vec[1].description, "Do homework");
    assert_eq!(vec[2].description, "Do more homework");
}

#[test]
fn date_time_string_format() {
    let dt1 = NaiveDateTime::parse_from_str("02/17/2020 23:00", "%m/%d/%Y %H:%M").unwrap();
    let dt2 = NaiveDateTime::parse_from_str("02/18/2020 00:45", "%m/%d/%Y %H:%M").unwrap();

    let formatted_dt1 = dt1.format("%m/%d/%Y %H:%M").to_string();
    let formatted_dt2 = dt2.format("%m/%d/%Y %H:%M").to_string();

    assert_eq!(formatted_dt1, "02/17/2020 23:00");
    assert_eq!(formatted_dt2, "02/18/2020 00:45");
}

#[test]
fn date_less_than() {
    let dt1 = NaiveDateTime::parse_from_str("02/04/2020 00:00", "%m/%d/%Y %H:%M").unwrap();
    let dt2 = NaiveDateTime::parse_from_str("02/03/2020 00:00", "%m/%d/%Y %H:%M").unwrap();

    let dt3 = NaiveDateTime::parse_from_str("02/29/2020 00:00", "%m/%d/%Y %H:%M").unwrap();
    let dt4 = NaiveDateTime::parse_from_str("02/01/2020 00:00", "%m/%d/%Y %H:%M").unwrap();

    assert!(dt2 < dt1);
    assert!(dt4 < dt3);
}

#[test]
fn time_less_than() {
    let dt1 = NaiveDateTime::parse_from_str("01/01/2020 14:30", "%m/%d/%Y %H:%M").unwrap();
    let dt2 = NaiveDateTime::parse_from_str("01/01/2020 14:25", "%m/%d/%Y %H:%M").unwrap();

    let dt3 = NaiveDateTime::parse_from_str("01/01/2020 23:59", "%m/%d/%Y %H:%M").unwrap();
    let dt4 = NaiveDateTime::parse_from_str("01/01/2020 00:00", "%m/%d/%Y %H:%M").unwrap();

    assert!(dt2 < dt1);
    assert!(dt4 < dt3);
}

#[test]
fn remove_owner_from_apptbook() {
    let mut apptbook: HashMap<String, Vec<Appointment>> = HashMap::new();

    let start_date_time1 = "03/11/2020 07:30";
    let end_date_time1 = "03/11/2020 08:30";

    let start_date_time2 = "03/18/2020 12:30";
    let end_date_time2 = "03/18/2020 12:40";

    let appt1 = Appointment {
        description: "Dentist appointment".to_string(),
        start_date_time: NaiveDateTime::parse_from_str(&start_date_time1, "%m/%d/%Y %H:%M")
            .unwrap(),
        end_date_time: NaiveDateTime::parse_from_str(&end_date_time1, "%m/%d/%Y %H:%M").unwrap(),
    };

    let appt2 = Appointment {
        description: "Eat Candy".to_string(),
        start_date_time: NaiveDateTime::parse_from_str(&start_date_time2, "%m/%d/%Y %H:%M")
            .unwrap(),
        end_date_time: NaiveDateTime::parse_from_str(&end_date_time2, "%m/%d/%Y %H:%M").unwrap(),
    };

    apptbook
        .entry("Billy".to_string())
        .or_insert_with(Vec::new)
        .push(appt1);

    apptbook
        .entry("Erik".to_string())
        .or_insert_with(Vec::new)
        .push(appt2);

    assert_eq!(apptbook.len(), 2);
    apptbook.remove("Billy");
    assert_eq!(apptbook.len(), 1);
    apptbook.remove("Erik");
    assert_eq!(apptbook.len(), 0);
}

#[test]
fn remove_appt_from_apptbook_vec() {
    let mut apptbook: HashMap<String, Vec<Appointment>> = HashMap::new();

    let start_date_time1 = "02/02/2020 11:30";
    let end_date_time1 = "02/02/2020 12:15";

    let start_date_time2 = "02/17/2020 11:00";
    let end_date_time2 = "02/17/2020 11:30";

    let appt1 = Appointment {
        description: "Have lunch with Lisa".to_string(),
        start_date_time: NaiveDateTime::parse_from_str(&start_date_time1, "%m/%d/%Y %H:%M")
            .unwrap(),
        end_date_time: NaiveDateTime::parse_from_str(&end_date_time1, "%m/%d/%Y %H:%M").unwrap(),
    };

    let appt2 = Appointment {
        description: "Eat lunch alone".to_string(),
        start_date_time: NaiveDateTime::parse_from_str(&start_date_time2, "%m/%d/%Y %H:%M")
            .unwrap(),
        end_date_time: NaiveDateTime::parse_from_str(&end_date_time2, "%m/%d/%Y %H:%M").unwrap(),
    };

    apptbook
        .entry("Tom".to_string())
        .or_insert_with(Vec::new)
        .push(appt1);

    apptbook
        .entry("Tom".to_string())
        .or_insert_with(Vec::new)
        .push(appt2);

    let toms_vec = apptbook.get_mut("Tom").unwrap();

    assert_eq!(toms_vec.len(), 2);
    toms_vec.remove(0);
    assert_eq!(toms_vec.len(), 1);
    assert_eq!(toms_vec[0].description, "Eat lunch alone");
}
