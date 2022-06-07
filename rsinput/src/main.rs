use std::{
    fs::File,
    io::{self, BufRead},
};
use regex::Regex;

fn main() {
    let filename = "input/key.txt";
    let file = File::open(filename).unwrap();
    let contents = io::BufReader::new(file).lines();

    for line in contents {
        if let Ok(msg) = line {
            let re = Regex::new(r"\\").unwrap();
            println!("{:?}", re.replace_all(&msg, "\x5b"));
            //     let new_msg = msg.escape_unicode().to_string();
            //     // let char_vec: Vec<char> = new_msg.chars().collect();
            //     // println!("{:x}", new_msg);
            //     println!("{:x}", "\\u{{2764}}");
        }
    }

    
}

