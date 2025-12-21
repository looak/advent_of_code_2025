// day_seven.rs
use std::fs;

#[derive(Debug, Eq, PartialEq)]
enum GridState {
    Empty,
    Splitter,
    Beam,
    Start,
    End,
}

fn load_grid() -> (Vec<GridState>, usize) {
    // 1. Read the file (imperative step, fine for I/O)
    let file_content = fs::read_to_string("./src/beam.txt")
        .expect("Unable to read file");

    let width = file_content
        .lines()
        .next()
        .unwrap()
        .len();

    // 2. The Functional Pipeline
    let grid = file_content
        .lines() // Iterator of &str (rows)
        .flat_map(|line| line.chars()) // Flattens: Row 1 chars -> Row 2 chars -> ...
        .map(|ch| match ch {
            // We RETURN the value, we do not push it.
            '.' => GridState::Empty,
            '^' => GridState::Splitter,
            '|' => GridState::Beam,
            'S' => GridState::Start,
            '\0' => GridState::End,
            _ => panic!("Unexpected character: {}", ch),
        })
        .collect(); // The compiler sees we return Vec<GridState> and builds it here.
    
    (grid, width)
}

pub fn execute() {
    println!("Hello Day Seven!");

    let (mut grid, width) = load_grid();

    // recursive depth first populate beams from splitters
    // find start
    let start_index = grid.iter().position(|state| *state == GridState::Start).expect("No start found");

    let mut beam_count = 0;
    populate_beams(&mut grid, &mut beam_count, width, start_index);

    // draw grid
    grid.iter().enumerate().for_each(|(indx, state)| {
        let local_indx: isize = indx as isize % width as isize;
        if local_indx == 0 && indx != 0 {
            println!();
        }
        let ch = match state {
            GridState::Empty => '.',
            GridState::Splitter => '^',
            GridState::Beam => '|',
            GridState::Start => 'S',
            GridState::End => '\0',
        };
        print!("{}", ch);
    });

    println!();

    println!("Total beam count: {}", beam_count);
    
}

fn populate_beams(grid: &mut Vec<GridState>, count: &mut usize, width: usize, index: usize) {
    // base case
    if index >= grid.len() {
        return;
    }

    match grid[index] {
        GridState::Start => {
            populate_beams(grid, count, width, index + width);
            return;
        }
        _ => {}
    }

    let local_indx: isize = index as isize % width as isize;

    // look back to propagate beams
    let prev_index = index - width;
    match grid[index] {
        GridState::Empty => {
            match grid[prev_index] {
                GridState::Beam => {
                    grid[index] = GridState::Beam;
                    populate_beams(grid, count, width, index + width);
                },
                GridState::Start => {
                    grid[index] = GridState::Beam;
                    populate_beams(grid, count, width, index + width);
                },
                _ => {}
            }
        },        
        _ => {}
    }

    match grid[index] {
        GridState::Splitter => {
            let mut local_split_count = 0;
            // propagate beam right & left
            if local_indx - 1 >= 0 {
                let left_index = index - 1;
                if grid[left_index] != GridState::Beam {
                    local_split_count += 1;
                    grid[left_index] = GridState::Beam;
                    populate_beams(grid, count, width, left_index + width);
                }
            }
            if local_indx + 1 < width as isize {
                let right_index = index + 1;
                if grid[right_index] != GridState::Beam {
                    if local_split_count == 0 {
                        local_split_count += 1;
                    }
                    grid[right_index] = GridState::Beam;                
                    populate_beams(grid, count, width, right_index + width);
                }
            }
            *count += local_split_count;
        },
        _ => {}
    }
}