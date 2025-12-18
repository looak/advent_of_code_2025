use std::fs::File;
use std::io::prelude::*;

fn load_lock_combination() -> Vec<String>
{
    // open file lock-combination.txt and read lines into a vector
    let file = std::fs::read_to_string("./src/lock-combination.txt").expect("Unable to read file");
    return file.lines().map(|line| line.to_string()).collect();
}

pub fn execute()
{
    println!("Hello, Day One!");
    let filename = "./output.log";
    let _ = std::fs::remove_file(filename); // remove file if it exists
    let mut file = File::create(filename).expect("Unable to create file");
    
    let mut cursor: i16 = 50;
    let mut count: i16 = 0;
    let mut prev_cursor: i16 = cursor;

    for instruction in load_lock_combination().iter()
    {
        let (turn, steps) = instruction.split_at(1);
        let mut steps: i16 = steps.parse().unwrap();
        let passed_zero = steps / 100;
        count += passed_zero;
        steps = steps % 100;
        match turn
        {
            "L" => cursor -= steps,
            "R" => cursor += steps,
            _ => (),
        }

        if cursor < 0
        {
            cursor += 100;
            if prev_cursor != 0
            {
                count += 1;
            }
        }
        else if cursor > 99
        {
            cursor -= 100;
            if prev_cursor != 0
            {
                count += 1;
            }
        }
        else if cursor == 0
        {
            count += 1;
        }
        
        prev_cursor = cursor;
        // output current cursor position and count to file
        file.write_all(format!("After instruction {}, cursor is at {}, count = {}\n", instruction, cursor, count).as_bytes()).expect("Unable to write data");

        println!("After instruction {}, cursor is at {}, count = {}", instruction, cursor, count);
    }
    println!("Number of times cursor was at 0: {}", count);
}
