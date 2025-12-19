fn load_batteries() -> Vec<String>
{
    // open file batteries.txt and read lines into a vector
    let file = std::fs::read_to_string("./src/batteries.dbg.txt").expect("Unable to read file");
    return file.lines().map(|line| line.to_string()).collect();
}

fn find_largest_capacity(battery: &String) -> u32
{
    // twp iterators, one for tens and one for ones.
    // tens has to be to the left of ones, i.e.
    let mut largest: u32 = 0;
    let mut tens_index: usize = 0;
    for (index, tens) in battery.chars().enumerate()
    {
        if index + 1 >= battery.len()
        {
            // last index can never be tens place.
            break;
        }
;
        let value = tens.to_digit(10).unwrap();
        if value > largest
        {
            largest = value;
            tens_index = index;
        }
    }

    let tens = battery.chars().nth(tens_index).unwrap().to_digit(10).unwrap();
    for index in tens_index+1..battery.len()
    {
        let ones = battery.chars().nth(index).unwrap().to_digit(10).unwrap();
        let value = tens * 10 + ones;
        if value > largest
        {
            largest = value;
        }
    } 
    return largest;
}

pub fn execute()
{
    println!("Hello Day Three!");
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