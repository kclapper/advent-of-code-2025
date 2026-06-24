use std::env;
use std::fs;

#[derive(Debug)]
enum Operation {
    MULTIPLY,
    ADD
}

#[derive(Debug)]
struct Problem {
    op: Operation,
    params: Vec<i64>
}

#[derive(Clone, Debug)]
struct Column(Vec<char>);
struct Input(Vec<Column>);
struct RawProblem(Vec<Column>);

fn file_to_input(file: &String) -> Input {
    let mut lines: Vec<&str> = file.lines().collect();
    let num_lines = lines.len();

    if lines[num_lines - 1].trim().len() == 0 {
        lines.remove(num_lines - 1);
    }

    let raw_input: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect();

    let num_columns = raw_input[0].len();
    let mut columns = Vec::with_capacity(num_columns);
    for col_num in 0..num_columns {
        columns.push(get_column(&raw_input, col_num));
    }

    return Input(columns);
}

fn get_column(file: &Vec<Vec<char>>, index: usize) -> Column {
    let mut column = Vec::with_capacity(file.len());
    for line in file {
        column.push(line[index]);
    }
    return Column(column);
}

fn parse_problems(file: Input) -> Vec<Problem> {
    collect_raw_problems(file)
        .iter()
        .map(|raw_problem| parse_problem(&raw_problem))
        .collect()
}

fn col_is_empty(col: &Column) -> bool {
    col.0.iter().fold(true, |acc, x| acc && x.is_whitespace())
}

fn collect_raw_problems(file: Input) -> Vec<RawProblem> {
    file.0
        .split(|col| col_is_empty(col))
        .map(|split| RawProblem(split.to_vec()))
        .collect()
}

fn parse_problem(raw: &RawProblem) -> Problem {
    let columns = &raw.0;
    let mut param_strings: Vec<String> = Vec::new();
    let mut op_string: String = String::new();

    let column_len = columns[0].0.len();
    for col in columns {
        let column = &col.0;
        let param_chars: String = column[0..column_len - 1].iter().collect();
        param_strings.push(String::from(param_chars));
        op_string.push(column[column_len - 1]);
    }

    let op = match op_string.trim() {
        "*" => Operation::MULTIPLY,
        "+" => Operation::ADD,
        _ => panic!("Unknown operation")
    };

    let params = param_strings
        .iter()
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    Problem { op, params }
}

fn evaluate_problems(problems: &Vec<Problem>) -> i64 {
    problems
        .iter()
        .fold(0, |acc, problem| {
            match problem.op {
                Operation::MULTIPLY => {
                    acc + problem.params.iter().fold(1, |acc, &x| acc * x)
                },
                Operation::ADD => {
                    acc + problem.params.iter().sum::<i64>()
                }
            }
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let file = fs::read_to_string(file_name).unwrap();
    let input = file_to_input(&file);
    let problems = parse_problems(input);

    let sum = evaluate_problems(&problems);

    println!("Final result: {}", sum);
}
