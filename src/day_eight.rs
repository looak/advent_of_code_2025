// day eight
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,

    closest_point_index: Option<usize>,
    part_of_circuit: bool,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z, closest_point_index: None, part_of_circuit: false }
    }

    // fn manhattan_distance(&self, other: &Point) -> i64 {
    //     ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as i64
    // }

    fn dist_sq(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz) as i64
    }

    fn distance(&self, other: &Point) -> f64 {
        (self.dist_sq(other) as f64).sqrt()
    }

    fn compare_and_set_closest(&mut self, points: &Vec<Point>, other: &Point, other_index: usize) {
        let new_distance_sq = self.dist_sq(other);
        match self.closest_point_index {
            Some(current_point_index) => {
                let current_point = &points[current_point_index];
                let current_distance_sq = self.dist_sq(current_point);
                if current_distance_sq < new_distance_sq {
                    self.closest_point_index = Some(other_index);
                }
            }
            None => {
                self.closest_point_index = Some(other_index);
            }
        }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

struct Circuit {
    points: Vec<Point>,
}

impl Circuit {
    fn new() -> Self {
        Self { points: Vec::new() }
    }
}

fn load_points() -> Vec<Point> {
    let file_content = fs::read_to_string("./src/electrical_junctions.txt")
        .expect("Unable to read file");

    let points: Vec<Point> = file_content
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .split(',')
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect();
            Point::new(coords[0], coords[1], coords[2])
        })
        .collect();

    points
}

fn compute_vicinity_table(points: &[Point]) -> Vec<usize> {
    let n = points.len();
    if n == 0 { return vec![]; }

    let vicinity = (0..n)
        .map(|i| {
            let mut best_dist = i64::MAX;
            let mut best_indx = 0;
            for j in 0..n {
                if i == j { continue; }
                let dist = points[i].dist_sq(&points[j]);
                if dist < best_dist {
                    best_dist = dist;
                    best_indx = j;
                }
            }
            best_indx
        }).collect();    

    vicinity
}

pub fn execute() {
    println!("Hello Day Eight!");
    let points = load_points();
    // let mut circuits = Circuit::new();

    let vicinity_table = compute_vicinity_table(&points);
    let mut parent: Vec<usize> = (0..points.len()).collect();

    // Standard Find with path compression
    fn find(parent: &mut [usize], i: usize) -> usize {
        if parent[i] == i { i } else {
            let root = find(parent, parent[i]);
            parent[i] = root;
            root
        }
    }

    // Standard Union
    fn union(parent: &mut [usize], i: usize, j: usize) {
        let root_i = find(parent, i);
        let root_j = find(parent, j);
        if root_i != root_j {
            parent[root_j] = root_i;
        }
    }

    // 2. Merge the sets based on the neighbor links
    for (point_idx, &neighbor_idx) in vicinity_table.iter().enumerate() {
        union(&mut parent, point_idx, neighbor_idx);
    }

    // Bucket points by their root parent
    let mut circuits: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();
    
    for i in 0..points.len() {
        let root = find(&mut parent, i);
        circuits.entry(root).or_default().push(i);
    }

    for (circuit_id, point_indices) in &circuits {
        println!("Circuit {}: Points {:?} - cnt: {}", circuit_id, point_indices, point_indices.len());
        // print points at indices
        points.iter()
            .enumerate()
            .filter(|(idx, _)| point_indices.contains(idx))
            .for_each(|(_, point)| println!("Point: {}-{}-{}", point.x, point.y, point.z));
        println!();
    }

//    println!("Maximum distance between any two points: {}", max_distance);
}