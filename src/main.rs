use std::env;
use std::fs::File;
use std::fs::*;
use std::io::Read;
use std::path::*;
fn read_lines(filename: &str) -> i32 {
    let mut result = Vec::new();
    let mut count: i32 = 0;
    let contents = match read_to_string(filename) {
        Ok(contents) => {
            for line in contents.lines() {
                result.push(line.to_string());
                count += 1;
            }
        }
        Err(error) => {
            eprintln!("Error reading file {}: {}", filename, error);
            return 0;
        }
    };

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = Path::new(&args[2]);
    let mut cur_file = match File::open(&file_path) {
        Err(why) => panic!("file at {:?} does not exist", why),
        Ok(cur_file) => cur_file,
    };

    let mut contents: String = String::new();
    //  println!("Printing args");
    //    println!("{}", args[0]);
    match cur_file.read_to_string(&mut contents) {
        Err(why) => panic!("Could'nt read the contents of {:?} into a string", why),

        Ok(_) => println!("Successfully read the contents of the file"),
    }

    let paths: &str = match file_path.to_str() {
        Some(paths) => paths,

        None => "coulnd print the file path",
    };

    if args[1] == "-l" {
        let line_wise_content: i32 = read_lines(&paths);
        println!("{}", line_wise_content);
    } else if args[1] == "-b" {
        let utf8_bytes = contents.as_bytes();

        let mut total_bytes: i32 = 0;

        for byte in utf8_bytes.iter() {
            total_bytes += 1;
        }
        println!("{}", total_bytes);
    } else if args[1] == "-w" {
        let utf8_bytes = contents.as_bytes();

        let mut total_words: i32 = 0;

        for word in utf8_bytes {
            if <u32 as Into<u32>>::into((*word).into()) == 32 {
                total_words += 1;
            }
        }
        println!("{}", total_words + 1);
    } else if args[1] == "-c" {
        let utf8_bytes = contents.as_bytes();

        let mut total_words: i32 = 0;

        for word in utf8_bytes {
            if <u32 as Into<u32>>::into((*word).into()) != 32 {
                total_words += 1;
            }
        }
        println!("{}", total_words - 1);
    }
    // for checking the args part of the code
}
