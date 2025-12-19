use std::thread;
use std::time::Duration;

fn load_warehouse() -> (Vec<bool>, usize)
{
    let mut result: Vec<bool> = Vec::new();
    // open file warehouse.dbg.txt and read lines into a vector
    let file = std::fs::read_to_string("./src/paper_rolls.txt").expect("Unable to read file");
    for line in file.lines() {
        println!("{}", line);
        result.push(false); // sentinal wall
        for char in line.chars() {
            if char == '@' {
                result.push(true);
            } else {
                result.push(false);
            }
        }
        result.push(false); // sentinal wall
    }
    let line_len = result.len() / file.lines().count();
    let mut grid: Vec<bool> = Vec::new();
    for _ in 0..line_len {
        grid.push(false); // sentinal wall
    }
    grid.append(&mut result);
    for _ in 0..line_len {
        grid.push(false); // sentinal wall
    }

    return (grid, file.lines().count()+2);
}

pub fn execute()
{
    println!("Hello Day Four !");
    let (mut back_warehouse, rows) = load_warehouse();
    let columns = back_warehouse.len() / rows;
    let adjecent_offsets: [isize; 8] = [
        -1, -1 - (columns as isize), -(columns as isize), 1 - (columns as isize),
        1, 1 + (columns as isize), (columns as isize), -1 + (columns as isize)
    ];

    let mut reachable_count = 0;
    let mut something_is_reachable = true;
    let mut front_warehouse = back_warehouse.clone();
    while something_is_reachable {
        let mut iteration_reachable_count = 0;
        let mut reachable_index: Vec<usize> = Vec::new();
        
        for (index, value) in front_warehouse.iter().enumerate() {
            if *value == false {
                continue;
            }
            else {
                let mut adjecent_count = 0;
                for offset in adjecent_offsets {
                    let neighbor_index: usize = (index as isize + offset) as usize;
                    if neighbor_index > back_warehouse.len() - 1 {
                        continue;
                    }
                    
                    if front_warehouse[neighbor_index] {
                        adjecent_count += 1;
                    }
                }

                if adjecent_count < 4 {
                    iteration_reachable_count += 1;
                    reachable_index.push(index);
                    back_warehouse[index] = false;
                }
            }
        }
    
        if iteration_reachable_count == 0 {
            something_is_reachable = false;
        }
        reachable_count += iteration_reachable_count;
        front_warehouse = back_warehouse.clone();
        
        //print!("\x1B[2J\x1B[1;1H");
        let mut buffer = String::new();
        buffer.push_str(&format!("New reachables: {}, Warehouse State:\n", iteration_reachable_count));
        for position in front_warehouse.iter().enumerate() {
            let (index, value) = position;
            
            if reachable_index.contains(&index) {
                buffer.push('x');
            } 
            else if *value == false {
                buffer.push('.');
            }
            else {
                buffer.push('@');
            }

            if (index + 1) % columns == 0 {
                buffer.push('\n');
            }
        }
        print!("{}", buffer);
        thread::sleep(Duration::from_millis(50));
        
    }
    println!("Warehouse loaded with {} items over {}x{} grid", back_warehouse.len(), rows, columns);
    println!("Reachable items count: {}", reachable_count);
}