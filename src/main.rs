#[macro_use] extern crate lazy_static;
extern crate csv;
extern crate regex;

use std::process::Command;
use regex::Regex;


fn main() {
    let fpath = ::std::env::args().nth(1).unwrap();

    // TODO: Run iconv when encoding is not utf8.
    println!("{}", get_encoding(&fpath));

    let mut rdr = csv::Reader::from_file(fpath).unwrap();
    let mut count = 0;


    for record in rdr.decode() {
        let row:Vec<String> = record.unwrap();

        // If we can't find the header in the first 30 rows
        // then chances are, it isn't here.
        if count == 30 {
            count = -1;
            break;
        }

        if is_header_row(&row) {
            break;
        }

         count += 1;
    }

    println!("{}", count);
}


/*
 * Given a file-path to a valid file, returns the encoding of that file
 * as determined by ```file -b --mime-encoding.```
 */
fn get_encoding(filename: &String) -> String {
    let output =
        Command::new("file")
            .args(&["-b", "--mime-encoding", filename])
            .output()
            .expect("failed to execute process");
    String::from_utf8(output.stdout).unwrap()
}


/*
 * For a given row (a vector of strings) this function looks for any
 * cells that start with department, amount, date, or entity. As long
 * as it finds at least three of them, it decides that it is the
 * header row.
 */
fn is_header_row(row: &Vec<String>) -> bool {
    lazy_static! {
        // Create once as required
        static ref RE:Regex = Regex::new(
            r"^(?i)(department*)|(date*)|(entity*)|(amount*)"
        ).unwrap();
    }


    let mut count = 0;
    for cell in row.iter() {
        if cell == "" {
            continue;
        }

        let cap = RE.captures(cell);
        match cap {
            Some(x) => {count = count + x.len();}
            None => {}
        }
    }
    count >= 3
}
