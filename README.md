# apptbook-rust 

### What is it?
*apptbook-rust* is a simple command-line appointment book program written in the Rust language. Its intended function is to provide the user with a place to store appointments for themself, as well as others. The program creates a mapping between appointment owners and their appointment books. Meaning each owner has their own appointment book containing one or more appointments. Right now the program allows the user to add, view, and delete appointments, but originally more features were planned.  

### How it works...
The program manages appointment info using an `Appointment` `struct`, which looks like the following:
```rust
struct Appointment {
    description: String,
    start_date_time: NaiveDateTime,
    end_date_time: NaiveDateTime,
}
```
The `NaiveDateTime` type comes from the `chrono` Rust crate. This type allows the program to perfrom calculations (such as sorting) on the date/time variables. The program ensures user input is successfully parsed into these types using date/time regular expressions, which are created with help from the `regex` crate. The actual date/time regexes were received from Stack Overflow. Links: [date](https://stackoverflow.com/questions/15491894/regex-to-validate-date-format-dd-mm-yyyy), [time](https://stackoverflow.com/questions/7536755/regular-expression-for-matching-hhmm-time-format/7536768).

A `HashMap` links an owner to a `Vec` of their appointments. Its type resembles the following:
```rust
HashMap<String, Vec<Appointment>>
```
Where the `String` represents the owner's name. Persistent storage of this data structure is achieved through the `serde` Rust crate. This crate gives the program the ability to serialize and deserialize the `HashMap`. The program writes the serialized `HashMap` out to file, and reads the deserialized `HashMap` from file. This process produces the "saving" of appointments.

### Building and Running...
Build with:
```
cargo build
```
Run with:
```
cargo run
```
Notable mentions:
* When prompted to enter a date by the program, accepted dates are in the format mm/dd/yyyy. Leading zeros are required. The regex to match this date accounts for leap years. For example:
  ```
  Enter the start date: 02/29/2019
  Invalid date. Required format: mm/dd/yyyy

  Enter the start date: 02/29/2020
  Enter the start time:
  ```
* When prompted to enter a time by the program, accepted times are in the format hh:mm. Leading zeros are required. This must be in 24-hour clock format. For example:
  ```
  Enter the start time: 24:00
  Invalid time. Required (24-hour clock) format: hh:mm

  Enter the start time: 23:59
  Enter the end date:
  ```

### Testing...
Testing was achieved by writing unit tests for principal operations of the program. Some of these operations include:  
* Regex parsing of date and times
* Serializing and deserializing the `HashMap`
* Sorting appointments by `start_date/time`
* Sorting appointments by `end_date/time` (if start date/times are equal)
* Sorting appointments by `description` (if end date/times are equal)
* Formatting `NaiveDateTime` into `String`
* Removal of keys (owners) from the `HashMap`
* Removal of appointments from the `Appointment` `Vec` in the `HashMap`

Run tests with:
```
cargo test
```

### What worked? What didn't?
Everything implemented at the moment seems to be working as intended. I'm satisfied with what I have so far, but would have liked to of had more features implemented. Here's a list:
- [x] adding appointments
- [x] viewing appointments
- [x] ordered appointments
- [x] removing appointments
- [x] persistent storage
- [ ] pretty printing appointments to file
- [ ] viewing appointments within a range
- [ ] cross viewing appointments between owners
