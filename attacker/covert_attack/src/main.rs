use chrono::prelude::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let size = 1000000000;
    let mut vec: Vec<i64> = vec![0; size];
    
    let file_path = "out.txt";
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating file: {}", err);
            return;
        }
    };

    let p_start = std::time::Instant::now();
    let mut c = 0;


    loop{
        let start_time = std::time::Instant::now();
        vec[c as usize % size] += 10;
        let elapsed_time = std::time::Instant::now() - start_time;
        let p_elaped_time = std::time::Instant::now() - p_start;

        match file.write_all(format!("{} {} {}\n", elapsed_time.as_nanos(), p_elaped_time.as_nanos(), Utc::now().second()).as_bytes()) {
            Ok(file) => file,
            Err(err) => eprintln!("Error writing to file: {}", err),
        }
        //println!("{} {} {}", elapsed_time.as_nanos(), p_elaped_time.as_nanos(), Local::now());
        c += 64;
        if p_elaped_time.as_secs() >= 10{
            break;
        }
    }
}
