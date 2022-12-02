use std::fs::File;
use std::io::{self, BufRead};

// Find the elf who has the most calories
// being held.
fn main() {

    let mut total: u32 = 0;
    let mut store: Vec<u32> = Vec::new();
    let file = File::open("./data/data.txt");
    for line in io::BufReader::new(file.expect("OK")).lines() {
        let l = line.expect("Ok").clone();
        let is_blank = l.is_empty();
        if is_blank {
            store.push(total);
            total = 0
        } else {
            total += l.parse::<u32>().expect("msg");
        }
    }

    store.sort();
    println!("Top Calories: {}", store.last().unwrap());
    let top_3_sum: u32 = store.as_slice()[store.len()-3..].iter().sum();
    println!("Top 3 Sum: {}", top_3_sum);
}
