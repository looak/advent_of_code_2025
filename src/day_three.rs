fn load_batteries() -> Vec<String>
{
    // open file batteries.txt and read lines into a vector
    let file = std::fs::read_to_string("./src/batteries.dbg.txt").expect("Unable to read file");
    return file.lines().map(|line| line.to_string()).collect();
}

fn find_largest_capacity(battery: &String) -> u64
{   
    let jolt_length = 12;
    let mut joltage: [char; 12] = ['0'; 12];
    let mut begin_index: u8 = 0;
    let mut end_index: u8 = battery.len() as u8 - jolt_length;
    for jolt_index in 0..12
    { 
        let mut largest: char = '0'; 
        for index in begin_index..=end_index
        {
            let largest_value = largest.to_digit(10).unwrap();
            let digit_char = battery.chars().nth(index as usize).unwrap();
            let value = digit_char.to_digit(10).unwrap();
            if value > largest_value
            {
                largest = digit_char;
                begin_index = index;
            }
        }

        joltage[jolt_index] = largest;
        begin_index += 1;
        end_index += 1;
    }
    let largest = joltage.iter().collect::<String>().parse::<u64>().unwrap();
    return largest;
}

pub fn execute()
{
    println!("Hello Day Three !");
    let batteries = load_batteries();

    let mut sum = 0;

    for battery in batteries.iter()
    {
        let largest = find_largest_capacity(battery);      
        sum += largest;
        println!("Battery: {} Value: {}", battery, largest);
    }
    println!("Total Sum of Largest Capacities: {}", sum);
}