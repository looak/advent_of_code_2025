use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {

    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn len(&self) -> u64 {
        (self.end - self.start) + 1
    }
}

fn load_database() -> (Vec<Range>, Vec<u64>) {
    // open file database.txt and read lines into a vector
    let file = std::fs::read_to_string("./src/ingredients.db").expect("Unable to read file");
    let mut ranges: Vec<Range> = Vec::new();
    let mut values: Vec<u64> = Vec::new();

    for line in file.lines() {
        if line.contains('-') {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            ranges.push(Range::new(start, end));
        } else if line.len() > 0
        {            
            let value: u64 = line.parse().unwrap();
            values.push(value);
        }
    }

    return (ranges, values);
}

pub fn execute() {
    println!("Hello Day Five!");
    let (mut ranges, _) = load_database();
    
    ranges.sort_by_key(|r| r.start);
    let mut merged: Vec<Range> = Vec::new();

    if let Some(first) = ranges.first() {
        merged.push(*first);
    }
    
    for next in ranges.iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if next.start <= last.end + 1 {
            last.end = cmp::max(last.end, next.end);
        } else {
            merged.push(*next);
        }
    }
    
    println!("Merged Ranges: {:?}", merged);
    
    let total_count: u64 = merged.iter().map(|r| r.len()).sum();
    println!("Total distinct integers: {}", total_count);
}