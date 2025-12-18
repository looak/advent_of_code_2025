fn load_ids() -> Vec<String>
{
    // open file ids.txt and read lines into a vector
    let file = std::fs::read_to_string("./src/db_ids_day_two.txt").expect("Unable to read file");
    // comma seperated values
    return file.lines().flat_map(|line| line.split(',').map(|s| s.to_string()).collect::<Vec<String>>()).collect();
}

fn has_pattern(pattern: &str, remaining: &str) -> bool
{
    let rem_end = remaining.len();
    let step = pattern.len();
    let mut itr = 0;

    while itr < rem_end
    {
        if itr + step > rem_end
        {
            return false;
        }

        let segment = &remaining[itr..itr + step];
        if segment != pattern
        {
            return false;
        }
        itr += step;
    }

    return true;
}

fn validate(number: u64) -> bool
{    
    // identify possible digit pairs in the id, 2 digits, 3 digits, 4 digits etc.
    // if a number contains 8 digits, 2s and 4s are the only valid repetitions.
    // if a number contains 9 diigits, only 3s are valid repetitions.
    // if a number contains 6 digits, 3s and 2s are valid repetitions.
    let num_str = number.to_string();
    let mut digit_count = 1;
    while digit_count <= num_str.len() / 2
    {
        let pattern = &num_str[0..digit_count];
        let remaining = &num_str[digit_count..];
        if has_pattern(pattern, remaining)
        {
            return false;
        }
        digit_count += 1;
    }

    return true;
}

pub fn execute()
{
    println!("Hello, Day Two!");
    let mut sum: u64 = 0;

    let ids = load_ids();
    for id in ids.iter()
    {
        println!("Loaded range: {}", id);
        let parts = id.split('-').collect::<Vec<&str>>();
        if parts.len() != 2
        {
            println!("Invalid range: {}", id);
            continue;
        }

        let range_start: u64 = parts[0].parse::<u64>().unwrap();
        let range_end = parts[1].parse::<u64>().unwrap();
        let mut itr = range_start;

        while itr <= range_end
        {
            if validate(itr) == false
            {
                println!("Invalid ID: {}", itr);
                sum += itr;
            }
            itr += 1;
        }
    }

    println!("Sum of valid IDs: {}", sum);
}