use chrono::prelude::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let size = 1000000000;
    let mut vec: Vec<i64> = vec![0; size];
    let secret = "1010010111";
    let mut i = 0;

    let file_path = "send.txt";
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            return;
        }
    };
    
    for bit in secret.chars(){
        let start_time = std::time::Instant::now();

        if bit == '1'{
            match file.write_all(format!("1 {}\n", Utc::now().second()).as_bytes()) {
                Ok(file) => file,
                Err(err) => eprintln!("Error writing to file: {}", err),
            }
            loop{
                vec[i % size] += 1;
                i += 62;
                let elapsed_time = std::time::Instant::now() - start_time;
                if elapsed_time.as_secs() >= 1{
                    break;
                }
            }
        } else {
            let start_time = std::time::Instant::now();
            match file.write_all(format!("0 {}\n", Utc::now().second()).as_bytes()) {
                Ok(file) => file,
                Err(err) => eprintln!("Error writing to file: {}", err),
            }
            loop{
                let elapsed_time = std::time::Instant::now() - start_time;
                if elapsed_time.as_secs() >= 1{
                    break;
                }
            }
        }
    }
}