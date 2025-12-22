// day_seven.rs
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum GridState {
    Empty,
    Splitter,
    Beam,
    Start,
    End,
}

struct GridNode {
    count: u64,
    state: GridState,
    fully_processed: bool,
}

impl GridNode {
    fn new(state: GridState) -> Self {
        Self {
            count: 0,
            state,
            fully_processed: false,
        }
    }
}

fn load_grid() -> (Vec<GridNode>, usize) {
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
            '.' => GridNode::new(GridState::Empty),
            '^' => GridNode::new(GridState::Splitter),
            '|' => GridNode::new(GridState::Beam),
            'S' => GridNode::new(GridState::Start),
            '\0' => GridNode::new(GridState::End),
            _ => panic!("Unexpected character: {}", ch),
        })
        .collect(); // The compiler sees we return Vec<GridState> and builds it here.
    
    (grid, width)
}

fn draw_grid(grid: &Vec<GridNode>, width: usize) {
    for (index, node) in grid.iter().enumerate() {
        if index % width == 0 && index != 0 {
            println!();
        }
        let ch = match node.state {
            GridState::Empty => '.',
            GridState::Splitter => '^',
            GridState::Beam => '|',
            GridState::Start => 'S',
            GridState::End => '\0',
        };
        print!("{}", ch);
    }
    println!();
}

pub fn execute() {
    println!("Hello Day Seven!");

    let (mut grid, width) = load_grid();

    // recursive depth first populate beams from splitters
    // find start
    let start_index = grid.iter().position(|grid_node| grid_node.state == GridState::Start).expect("No start found");

    let beam_count = populate_beams(&mut grid, width, start_index);

    // draw grid
    // draw_grid(&grid, width);

    println!();

    println!("Total beam count: {}", beam_count);
    
}

fn populate_beams(grid: &mut Vec<GridNode>, width: usize, index: usize) -> u64 {
    // base case
    if index >= grid.len() {
        return 1;
    }

    if (grid[index].fully_processed) {
        return grid[index].count;
    }

    if !matches!(grid[index].state, GridState::Splitter) {
        grid[index].state = GridState::Beam;
    }

    // Process Logic based on CURRENT state only
    let mut paths_found = 0;
    
    match grid[index].state {
        GridState::Splitter => {
            let local_col = (index % width) as isize;

            // --- Go Left-Down ---
            // Note: index - 1 is safe only if local_col > 0
            if local_col > 0 {
                let left_target = (index - 1) + width;
                paths_found += populate_beams(grid, width, left_target);
            }

            // --- Go Right-Down ---
            if local_col + 1 < width as isize {
                let right_target = (index + 1) + width;
                paths_found += populate_beams(grid, width, right_target);
            }
        },
        _ => {
            // Standard Beam/Start/Empty: Just fall strictly down
            paths_found += populate_beams(grid, width, index + width);
        }
    }

    // 5. Update State & Return
    grid[index].count += paths_found;
    grid[index].fully_processed = true;
    
    paths_found // Return the value explicitly!
}