#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Problem {
    terms: Vec<i64>,
    operator: Option<Operation>,
}

fn parse_line(line: &str) -> Vec<i64> {
    line
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn load_problems() -> Vec<Problem> {
    let file = std::fs::read_to_string("./src/math_problems.db").expect("Unable to read file");

    let mut lines = file.lines();
    let first_line = lines.next().expect("File is empty");
    let first_row = parse_line(&first_line);

    let mut problems: Vec<Problem> = first_row
        .into_iter()
        .map(|part| Problem {
            terms: vec![part],
            operator: None,
        })
        .collect();

    for line in lines {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        // peek at first character to determine if row is operators or terms
        let first_char = line.chars().next().unwrap();
        if first_char.is_ascii_digit() {
            let row_terms = parse_line(&line);
            
            if row_terms.len() != problems.len() {
                panic!("Data mismatch! Expected {} columns, found {}", problems.len(), row_terms.len());
            }

            for (problem, term) in problems.iter_mut().zip(row_terms) {
                problem.terms.push(term);
            }
        } else {
            let row_operators: Vec<Operation> = line
                .split_whitespace()
                .map(|op_str| match op_str {
                    "+" => Operation::Add,
                    "-" => Operation::Subtract,
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    _ => panic!("Unknown operator: {}", op_str),
                })
                .collect();

            if row_operators.len() != problems.len() {
                panic!("Data mismatch! Expected {} columns, found {}", problems.len(), row_operators.len());
            }

            for (problem, operator) in problems.iter_mut().zip(row_operators) {
                problem.operator = Some(operator);
            }
        }
    }

    return problems;
}

pub fn execute() {
    let problems = load_problems();
    let result = problems.iter().map(|problem| 
        match problem.operator.expect("Operator missing for problem") {
            Operation::Add => problem.terms.iter().sum::<i64>(),
            Operation::Multiply => problem.terms.iter().product::<i64>(),
            Operation::Subtract => todo!("Implement subtraction"),
            Operation::Divide => todo!("Implement division"),
    }).sum::<i64>();

    println!("Result: {}", result);
}