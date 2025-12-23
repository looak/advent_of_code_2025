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

#[derive(Debug, Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    len_sq: i64,
}

impl Edge {
    fn new(u: usize, v: usize, len_sq: i64) -> Self {
        Self { u, v, len_sq }
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

fn compute_edges(points: &[Point]) -> Vec<Edge> {
    let n = points.len();
    if n == 0 { return vec![]; }

    let mut edges: Vec<Edge> = Vec::with_capacity(n*n/2);
    
    for i in 0..n {
        for j in (i + 1)..n {
            let point_a = &points[i];
            let point_b = &points[j];
            let dist_sq = point_a.dist_sq(point_b);
            edges.push(Edge::new(i, j, dist_sq));
        }
    }

    edges
}

pub fn execute() {
    println!("Hello Day Eight!");
    let points = load_points();
    // let mut circuits = Circuit::new();

    let mut edges = compute_edges(&points);
    let mut parent: Vec<usize> = (0..points.len()).collect();
    edges.sort_unstable_by_key(|e| e.len_sq);

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

    let limit = 1000;
    for edge in edges.iter().take(limit) {
        union(&mut parent, edge.u, edge.v);
    }

    // Bucket points by their root parent
    let mut circuits: std::collections::HashMap<usize, Vec<usize>> = std::collections::HashMap::new();    
    for i in 0..points.len() {
        let root = find(&mut parent, i);
        circuits.entry(root).or_default().push(i);
    }

    let mut largest: [(usize, usize); 3] = [(0, 0); 3];
    for (&circuit_id, point_indices) in &circuits {
        if point_indices.len() > largest[0].1 {
            largest[2] = largest[1];
            largest[1] = largest[0];
            largest[0] = (circuit_id, point_indices.len());
        } else if point_indices.len() > largest[1].1 {
            largest[2] = largest[1];
            largest[1] = (circuit_id, point_indices.len());
        } else if point_indices.len() > largest[2].1 {
            largest[2] = (circuit_id, point_indices.len());
        }
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

    let mut result = 1;
    for circuit in &largest {
        println!("Largest Circuit ID: {} with {} points", circuit.0, circuit.1);
        result *= circuit.1;
    }   

    println!("Product of sizes of three largest circuits: {}", result);
}