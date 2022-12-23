use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn report(file: &String, ccount: u32, wcount: u32, lcount: u32) {
    println!("{:8}{:8}{:8} {}", lcount, wcount, ccount, file);
}

// fn get_word(f: &mut File, ccount: &mut u32, wcount: &mut u32, lcount: &mut u32) -> bool {
//     let mut iter = f.bytes().peekable();
//     if iter.peek().is_none() {
//         return false;
//     }

//     let mut byte = 0;

//     loop {
//         if iter.peek().is_none() {
//             break;
//         }
//         byte = iter.next().unwrap().unwrap();
//         if !char::is_ascii_whitespace(&char::from_u32(byte as u32).unwrap()) {
//             *wcount += 1;
//             break;
//         }
//         *ccount += 1;
//         if byte == b'\n' {
//             *lcount += 1;
//         }
//     }

//     loop {
//         *ccount += 1;
//         if byte == b'\n' {
//             *lcount += 1;
//         }
//         if char::is_ascii_whitespace(&char::from_u32(byte as u32).unwrap()) {
//             break;
//         }
//         // println!("{:#?}", byte as char);
//         if iter.peek().is_none() {
//             break;
//         }
//         byte = iter.next().unwrap().unwrap();
//     }
    
//     iter.peek().is_some()
// }

fn counter(filename: &String, tot_ccount: &mut u32, tot_wcount: &mut u32, tot_lcount: &mut u32) {
    let f = io::BufReader::new(File::open(filename).unwrap());

    let mut ccount = 0u32;
    let mut wcount = 0u32;
    let mut lcount = 0u32;

    for line in f.lines() {
        lcount += 1;
        ccount += 1;
        let line = line.unwrap();
        let mut is_in_word = false;
        for ch in line.chars() {
            ccount += 1;
            if !ch.is_ascii_whitespace() {
                if !is_in_word {
                    wcount += 1;
                    is_in_word = true;
                }
            } else {
                is_in_word = false;
            }
        }
    }

    *tot_ccount += ccount;
    *tot_wcount += wcount;
    *tot_lcount += lcount;
    report(filename, ccount, wcount, lcount);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        println!("Usage: rwc file [file...]");
        process::exit(1);
    }
    
    let mut tot_ccount = 0u32;
    let mut tot_wcount = 0u32;
    let mut tot_lcount = 0u32;
    
    for filename in args.iter().skip(1) {
        counter(filename, &mut tot_ccount, &mut tot_wcount, &mut tot_lcount);
    }
    if args.len() > 2 {
        report(&String::from("total"), tot_ccount, tot_wcount, tot_lcount);
    }
}
