use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
struct Dial {
    position: i32,
    on_zero_count: i32,
    pass_zero_count: i32
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, PartialEq)]
struct Turn {
    direction: Direction,
    steps: i32,
    passes: i32
}

fn parse_turn(line: &str) -> Turn {
    let raw_steps = line[1..].parse::<u16>().unwrap();
    Turn {
        direction: match &line[0..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!()
        },
        steps: (raw_steps % 100) as i32,
        passes: (raw_steps / 100) as i32
    }
}

fn turn_right(dial: Dial, turn: &Turn) -> Dial {
    let new_position = (dial.position + turn.steps) % 100;
    Dial {
        position: new_position,
        on_zero_count: if new_position == 0 {
            dial.on_zero_count + 1
        } else {
            dial.on_zero_count
        },
        pass_zero_count: if new_position < dial.position && new_position != 0 {
            dial.pass_zero_count + turn.passes + 1
        } else {
            dial.pass_zero_count + turn.passes
        }
    }
}

fn turn_left(dial: Dial, turn: &Turn) -> Dial {
    let mut new_position = dial.position - turn.steps;
    new_position = if new_position >= 0 {
        new_position
    } else {
        100 + new_position
    };

    Dial {
        position: new_position,
        on_zero_count: if new_position == 0 {
            dial.on_zero_count + 1
        } else {
            dial.on_zero_count
        },
        pass_zero_count: if new_position > dial.position && dial.position != 0 {
            dial.pass_zero_count + turn.passes + 1
        } else {
            dial.pass_zero_count + turn.passes
        }
    }
}

fn take_turn(dial: Dial, turn: &Turn) -> Dial {
    match turn.direction {
        Direction::Left => turn_left(dial, turn),
        Direction::Right => turn_right(dial, turn),
    }
}

fn fold_line(dial: Dial, line: &str) -> Dial {
    take_turn(dial, &parse_turn(line))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let dial = fs::read_to_string(file_name)
        .expect("Could not read input")
        .lines()
        .fold(Dial {
            position: 50,
            on_zero_count: 0,
            pass_zero_count: 0
        }, fold_line);

    print!("Position: {}\n", dial.position);
    print!("On zero count: {}\n", dial.on_zero_count);
    print!("Pass zero count: {}\n", dial.pass_zero_count);
    print!("Total hits on zero: {}\n", dial.on_zero_count + dial.pass_zero_count);
}

#[cfg(test)]
mod tests {
    use std::mem::take;

    use super::*;

    #[test]
    fn parse_turn_test() {
        assert_eq!(
            parse_turn("L11"),
            Turn {
                direction: Direction::Left,
                steps: 11,
                passes: 0
            }
        );

        assert_eq!(
            parse_turn("R11"),
            Turn {
                direction: Direction::Right,
                steps: 11,
                passes: 0
            }
        );

        assert_eq!(
            parse_turn("R0"),
            Turn {
                direction: Direction::Right,
                steps: 0,
                passes: 0
            }
        );

        assert_eq!(
            parse_turn("L0"),
            Turn {
                direction: Direction::Left,
                steps: 0,
                passes: 0
            }
        );

        assert_eq!(
            parse_turn("L100"),
            Turn {
                direction: Direction::Left,
                steps: 0,
                passes: 1
            }
        );

        assert_eq!(
            parse_turn("R100"),
            Turn {
                direction: Direction::Right,
                steps: 0,
                passes: 1
            }
        );

        assert_eq!(
            parse_turn("R200"),
            Turn {
                direction: Direction::Right,
                steps: 0,
                passes: 2
            }
        );
    }

    #[test]
    fn take_turn_test() {
        assert_eq!(
            take_turn(Dial {
                position: 0,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Right,
                steps: 1,
                passes: 0
            }),
            Dial {
                position: 1,
                on_zero_count: 0,
                pass_zero_count: 0
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 0,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Left,
                steps: 1,
                passes: 0
            }),
            Dial {
                position: 99,
                on_zero_count: 0,
                pass_zero_count: 0
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 99,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Right,
                steps: 1,
                passes: 0
            }),
            Dial {
                position: 0,
                on_zero_count: 1,
                pass_zero_count: 0
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 0,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Right,
                steps: 0,
                passes: 1
            }),
            Dial {
                position: 0,
                on_zero_count: 1,
                pass_zero_count: 1
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 0,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Left,
                steps: 0,
                passes: 1
            }),
            Dial {
                position: 0,
                on_zero_count: 1,
                pass_zero_count: 1
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 50,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Left,
                steps: 50,
                passes: 0
            }),
            Dial {
                position: 0,
                on_zero_count: 1,
                pass_zero_count: 0
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 50,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Right,
                steps: 50,
                passes: 0
            }),
            Dial {
                position: 0,
                on_zero_count: 1,
                pass_zero_count: 0
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 50,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Right,
                steps: 0,
                passes: 1
            }),
            Dial {
                position: 50,
                on_zero_count: 0,
                pass_zero_count: 1
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 50,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Left,
                steps: 0,
                passes: 1
            }),
            Dial {
                position: 50,
                on_zero_count: 0,
                pass_zero_count: 1
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 50,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Right,
                steps: 0,
                passes: 0
            }),
            Dial {
                position: 50,
                on_zero_count: 0,
                pass_zero_count: 0
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 90,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Right,
                steps: 20,
                passes: 0
            }),
            Dial {
                position: 10,
                on_zero_count: 0,
                pass_zero_count: 1
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 10,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Left,
                steps: 20,
                passes: 0
            }),
            Dial {
                position: 90,
                on_zero_count: 0,
                pass_zero_count: 1
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 0,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Right,
                steps: 99,
                passes: 0
            }),
            Dial {
                position: 99,
                on_zero_count: 0,
                pass_zero_count: 0
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 99,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Left,
                steps: 99,
                passes: 0
            }),
            Dial {
                position: 0,
                on_zero_count: 1,
                pass_zero_count: 0
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 90,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Right,
                steps: 20,
                passes: 4
            }),
            Dial {
                position: 10,
                on_zero_count: 0,
                pass_zero_count: 5
            }
        );

        assert_eq!(
            take_turn(Dial {
                position: 10,
                on_zero_count: 0,
                pass_zero_count: 0
            }, &Turn {
                direction: Direction::Left,
                steps: 20,
                passes: 4
            }),
            Dial {
                position: 90,
                on_zero_count: 0,
                pass_zero_count: 5
            }
        );
    }

    #[test]
    fn test_example_2() {
        let mut dial = Dial {
            position: 50,
            on_zero_count: 0,
            pass_zero_count: 0
        };

        dial = take_turn(dial, &parse_turn("L68"));
        assert_eq!(dial, Dial {
            position: 82,
            on_zero_count: 0,
            pass_zero_count: 1
        });

        dial = take_turn(dial, &parse_turn("L30"));
        assert_eq!(dial, Dial {
            position: 52,
            on_zero_count: 0,
            pass_zero_count: 1
        });

        dial = take_turn(dial, &parse_turn("R48"));
        assert_eq!(dial, Dial {
            position: 0,
            on_zero_count: 1,
            pass_zero_count: 1
        });

        dial = take_turn(dial, &parse_turn("L5"));
        assert_eq!(dial, Dial {
            position: 95,
            on_zero_count: 1,
            pass_zero_count: 1
        });

        dial = take_turn(dial, &parse_turn("R60"));
        assert_eq!(dial, Dial {
            position: 55,
            on_zero_count: 1,
            pass_zero_count: 2
        });

        dial = take_turn(dial, &parse_turn("L55"));
        assert_eq!(dial, Dial {
            position: 0,
            on_zero_count: 2,
            pass_zero_count: 2
        });

        dial = take_turn(dial, &parse_turn("L1"));
        assert_eq!(dial, Dial {
            position: 99,
            on_zero_count: 2,
            pass_zero_count: 2
        });

        dial = take_turn(dial, &parse_turn("L99"));
        assert_eq!(dial, Dial {
            position: 0,
            on_zero_count: 3,
            pass_zero_count: 2
        });

        dial = take_turn(dial, &parse_turn("R14"));
        assert_eq!(dial, Dial {
            position: 14,
            on_zero_count: 3,
            pass_zero_count: 2
        });

        dial = take_turn(dial, &parse_turn("L82"));
        assert_eq!(dial, Dial {
            position: 32,
            on_zero_count: 3,
            pass_zero_count: 3
        });

        assert_eq!(dial.on_zero_count + dial.pass_zero_count, 6);
    }

    #[test]
    fn test_example_3() {
        let mut dial = Dial {
            position: 50,
            on_zero_count: 0,
            pass_zero_count: 0
        };

        dial = take_turn(dial, &parse_turn("R1000"));
        assert_eq!(dial, Dial {
            position: 50,
            on_zero_count: 0,
            pass_zero_count: 10
        });
    }
}
