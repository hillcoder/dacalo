
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::env;
use rand::Rng;
use std::path::Path;

fn set_caps(ch : char) -> char {

    if ch.is_alphabetic() {
        if rand::thread_rng().gen_bool(0.5) {
            if ch.is_uppercase(){
                return ch.to_ascii_lowercase();
            } else {
                return ch.to_ascii_uppercase();
            }
        }
    }
    return ch;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        println!("Damaged Caps Lock");
        println!("Usage:");
        println!("{} <filename>",&args[0]);        
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        println!("Unable to open {} file",&file_path);        
        return;
    }

    let file = BufReader::new(File::open(file_path).expect("Unable to open file"));

    let mut a_string = String::from("");

    for line in file.lines() {        
        for ch in line.expect("Unable to read line").chars() {
            a_string.push(set_caps(ch));   
        }
        a_string.push('\n');
    }

    println!("{}",a_string);

}
