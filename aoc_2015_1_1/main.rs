use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut ans = 0;
    let mut found = false;
    for line in reader.lines() {
        let line = line?;
        for (i, c) in line.chars().enumerate() {
            if c == '(' {
                ans += 1;
            } else if c == ')' {
                ans -= 1;
            }

            if ans == -1 && !found {
                println!("Ind: {}", i + 1);
                found = true;
            }
        }
    }
    println!("Ans: {ans}");

    Ok(())
}

