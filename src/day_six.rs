use std::{collections::BTreeMap, ops::IndexMut};

#[derive(Debug)]
struct Problem {
    terms: Vec<i64>,
    operator: Operation,
}

struct IntermediateProblem {
    terms: Vec<i64>,
    operator: Option<Operation>,
}

#[derive(Debug, Clone, Copy)] // Make sure Operation is Copy for easier handling
enum Operation { Add, Multiply }

struct ProblemBuilder {
    problem_data: BTreeMap<usize, ColumnData>,
}

#[derive(Debug)] 
struct ColumnData {
    digits: String,
    operator: Option<Operation>,
}

impl ProblemBuilder {
    fn new() -> Self {
        Self {
            problem_data: BTreeMap::new(),
        }
    }

    fn parse_line(&mut self, line: &str) {
        for (index, ch) in line.char_indices() {
            if ch.is_whitespace() {
                continue;
            }

            let column = self.problem_data.entry(index).or_insert(ColumnData {
                digits: String::new(),
                operator: None,
            });

            if ch.is_ascii_digit() {
                column.digits.push(ch);
            } else if ch == '+' {
                column.operator = Some(Operation::Add);
            } else if ch == '*' {
                column.operator = Some(Operation::Multiply);
            } else {
                panic!("Unexpected character in input: {}", ch);
            }
        }
    }

    fn collect(self) -> Vec<Problem> {
        let mut current_operator: Option<Operation> = None;
        let mut problems: Vec<Problem> = Vec::new();
        let mut problem_id = 0;
        // let mut problems: Vec<Problem> = Vec::new();
        for (indx, col_data) in &self.problem_data {
            println!("Column {}: {:?}", indx, col_data);
            
            if let Some(op) = col_data.operator {
                current_operator = Some(op);
                problem_id += 1;
            }

            let problem = problems.get_mut(problem_id - 1);
            if problem.is_none() {
                problems.push(Problem {
                    terms: Vec::new(),
                    operator: current_operator.unwrap(),
                });
            }

            if !col_data.digits.is_empty() {
                let term: i64 = col_data.digits.parse().expect("Failed to parse digits into integer");
                problems.index_mut(problem_id - 1).terms.push(term);
            }

        }

        return problems;

    }
}

fn load_problems() -> Vec<Problem> {
    let file = std::fs::read_to_string("./src/math_problems.db").expect("Unable to read file");
    let mut problems_builder = ProblemBuilder::new();
    for line in file.lines() {
        problems_builder.parse_line(line);
    }

    // Transform the vertical strings into actual integers
    problems_builder.collect()
}

pub fn execute() {
    println!("Hello Day Six!");
    let problems = load_problems();
    problems.iter().for_each(|p| println!("{:?}", p));
    let result = problems.iter().map(|problem| 
        match problem.operator {
            Operation::Add => problem.terms.iter().sum::<i64>(),
            Operation::Multiply => problem.terms.iter().rev().product::<i64>(),

    }).sum::<i64>();

    println!("Result: {}", result);
}