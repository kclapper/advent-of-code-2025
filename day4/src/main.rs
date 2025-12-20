use std::env;
use std::fs;

enum Space {
    Empty,
    Roll(u32)
}

fn is_left_edge(index: usize, columns: usize) -> bool {
    index % columns == 0
}

fn is_right_edge(index: usize, columns: usize) -> bool {
    index % columns == columns - 1
}

fn is_top_edge(index: usize, columns: usize) -> bool {
    index < columns
}

fn is_bottom_edge(index: usize, columns: usize, board_size: usize) -> bool {
    index + columns >= board_size
}

fn parse_board(input: &str) -> (Vec<Space>, usize) {
    let columns = input.lines().take(1).next().unwrap().trim().len();
    let mut board: Vec<Space> = Vec::with_capacity(input.len());

    let mut index = 0;
    for current in input.trim().chars() {
        if current.is_whitespace() {
            continue;
        }

        board.push(match current {
            '@' => Space::Roll(0),
            _ => Space::Empty
        });

        let space = &mut board[index];
        if let Space::Empty = space {
            index += 1;
            continue;
        }

        let mut neighbors = 0;

        if !is_left_edge(index, columns) {
            if let Space::Roll(their_neighbors) = board[index - 1] {
                neighbors += 1;
                board[index - 1] = Space::Roll(their_neighbors + 1);
            }
        }

        if !is_top_edge(index, columns) {
            if let Space::Roll(their_neighbors) = board[index - columns] {
                neighbors += 1;
                board[index - columns] = Space::Roll(their_neighbors + 1);
            }
        }

        if !is_left_edge(index, columns) && !is_top_edge(index, columns) {
            if let Space::Roll(their_neighbors) = board[index - columns - 1] {
                neighbors += 1;
                board[index - columns - 1] = Space::Roll(their_neighbors + 1);
            }
        }

        if !is_right_edge(index, columns) && !is_top_edge(index, columns) {
            if let Space::Roll(their_neighbors) = board[index - columns + 1] {
                neighbors += 1;
                board[index - columns + 1] = Space::Roll(their_neighbors + 1);
            }
        }

        board[index] = Space::Roll(neighbors);
        index += 1;
    } 

    return (board, columns);
}

fn trim_board(board: &mut Vec<Space>, columns: usize) {
    let mut todo: Vec<usize> = (0..board.len()).collect();
    while !todo.is_empty() {
        let index = todo.pop().unwrap();
        let space = &board[index];

        if let Space::Roll(neighbors) = *space {
            if neighbors >= 4 {
                continue;
            }
            
            board[index] = Space::Empty;

            if !is_left_edge(index, columns) {
                let neighbor = index - 1;
                if let Space::Roll(their_neighbors) = board[neighbor] {
                    board[neighbor] = Space::Roll(their_neighbors - 1);
                    todo.push(neighbor);
                }
            }

            if !is_right_edge(index, columns) {
                let neighbor = index + 1;
                if let Space::Roll(their_neighbors) = board[neighbor] {
                    board[neighbor] = Space::Roll(their_neighbors - 1);
                    todo.push(neighbor);
                }
            }

            if !is_top_edge(index, columns) {
                let neighbor = index - columns;
                if let Space::Roll(their_neighbors) = board[neighbor] {
                    board[neighbor] = Space::Roll(their_neighbors - 1);
                    todo.push(neighbor);
                }
            }

            if !is_left_edge(index, columns) && !is_top_edge(index, columns) {
                let neighbor = index - columns - 1;
                if let Space::Roll(their_neighbors) = board[neighbor] {
                    board[neighbor] = Space::Roll(their_neighbors - 1);
                    todo.push(neighbor);
                }
            }

            if !is_right_edge(index, columns) && !is_top_edge(index, columns) {
                let neighbor = index - columns + 1;
                if let Space::Roll(their_neighbors) = board[neighbor] {
                    board[neighbor] = Space::Roll(their_neighbors - 1);
                    todo.push(neighbor);
                }
            }

            if !is_bottom_edge(index, columns, board.len()) {
                let neighbor = index + columns;
                if let Space::Roll(their_neighbors) = board[neighbor] {
                    board[neighbor] = Space::Roll(their_neighbors - 1);
                    todo.push(neighbor);
                }
            }

            if !is_bottom_edge(index, columns, board.len()) && !is_left_edge(index, columns) {
                let neighbor = index + columns - 1;
                if let Space::Roll(their_neighbors) = board[neighbor] {
                    board[neighbor] = Space::Roll(their_neighbors - 1);
                    todo.push(neighbor);
                }
            }

            if !is_bottom_edge(index, columns, board.len()) && !is_right_edge(index, columns) {
                let neighbor = index + columns + 1;
                if let Space::Roll(their_neighbors) = board[neighbor] {
                    board[neighbor] = Space::Roll(their_neighbors - 1);
                    todo.push(neighbor);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let file = fs::read_to_string(file_name).unwrap();
    let (mut board, columns) = parse_board(&file);

    let rolls = board.iter().fold(0, |acc, space| {
        if let Space::Roll(_) = *space {
            acc + 1
        } else {
            acc
        }
    });

    trim_board(&mut board, columns);

    let rolls_left = board.iter().fold(0, |acc, space| {
        if let Space::Roll(_) = *space {
            acc + 1
        } else {
            acc
        }
    });

    println!("Gettable: {}", rolls - rolls_left);
}

#[cfg(test)]
mod tests {
    use super::*;


}