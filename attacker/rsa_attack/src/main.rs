fn main() {
    let size = 10000000;
    let mut vec: Vec<i64> = vec![2; 10000000];
    for x in 0..5000{
        for i in (0..size).step_by(12){
            let start_time = std::time::Instant::now();
            vec[i] = x;
            let end_time = std::time::Instant::now();
            let elapsed_time = end_time - start_time;
            println!("{}", elapsed_time.as_nanos());
        }
    }
}

//time total time