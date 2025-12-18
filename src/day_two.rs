fn load_ids() -> Vec<String>
{
    // open file ids.txt and read lines into a vector
    let file = std::fs::read_to_string("./src/db_ids_day_two.txt").expect("Unable to read file");
    // comma seperated values
    return file.lines().flat_map(|line| line.split(',').map(|s| s.to_string()).collect::<Vec<String>>()).collect();
}

fn validate(itr: u64, len: usize) -> bool
{    
    if len % 2 > 0
    {
        // length must be even
        return false;
    }

    let mut id = itr;
    let mut half = len / 2;
    
    // store first half digits in a vector
    let mut digits:Vec<u64> = Vec::new();
    while half > 0
    {
        digits.push(id % 10);
        id /= 10;
        half -= 1;
    }

    // check second half digits against first half
    for digit in &digits
    {
        let remainder = id % 10;
        if *digit != remainder
        {
            return false;
        }
        id /= 10;
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
            let len = itr.to_string().len();
            if validate(itr, len)
            {
                println!("Valid ID: {}", itr);
                sum += itr;
            }
            itr += 1;
        }

    }

    println!("Sum of valid IDs: {}", sum);
}