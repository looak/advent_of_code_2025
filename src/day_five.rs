
struct Range {
    start: u64,
    end: u64,
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
            ranges.push(Range { start, end });
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
    let (ranges, values) = load_database();
    let mut fresh_count = 0;
    for value in values.iter() {
        let mut in_range = false;
        for range in ranges.iter() {
            if *value >= range.start && *value <= range.end {
                in_range = true;
                break;
            }
        }
        if in_range {
            fresh_count += 1;
        }
    }
    println!("Number of fresh values: {}", fresh_count);
}