use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
struct Bank {
    batteries: Vec<u32>
}

impl Bank {
    fn max_joltage(&self) -> u32 {
        let mut first = self.batteries[0];
        let mut second = self.batteries[1];
        let mut max = (first * 10) + second;

        for current in self.batteries.iter().skip(2) {
            let shift = (second * 10) + *current;
            if shift > max {
                first = second;
                second = *current;
                max = shift;
            } else if *current > second {
                second = *current;
                max = (first * 10) + second;
            }
        }
        
        return max;
    }

    fn parse_line(line: &str) -> Bank {
        let mut values = Vec::with_capacity(64);
        for current in line.chars() {
            values.push(current.to_digit(10).unwrap());
        }

        Bank {
            batteries: values
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let total_joltage = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .fold(0, |acc, line| {
            acc + Bank::parse_line(line).max_joltage()
        });


    println!("Total joltage: {}", total_joltage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_bank_joltage() {
        // 987654321111111 - 98
        // 8,1,1,1,1,1,1,1,1,1,1,1,1,1,9 - 89
        // 2,3,4,2,3,4,2,3,4,2,3,4,2,7,8 - 78
        // 8,1,8,1,8,1,9,1,1,1,1,2,1,1,1 - 92

        let mut bank = Bank {
            batteries: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]
        };
        assert_eq!(bank.max_joltage(), 98);

        bank = Bank {
            batteries: vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9]
        };
        assert_eq!(bank.max_joltage(), 89);

        bank = Bank {
            batteries: vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]
        };
        assert_eq!(bank.max_joltage(), 78);

        bank = Bank {
            batteries: vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]
        };
        assert_eq!(bank.max_joltage(), 92);
    }

    #[test]
    fn parse_bank() {
        // 987654321111111 - 98
        // 811111111111119 - 89
        // 234234234234278 - 78
        // 818181911112111 - 92

        assert_eq!(
            Bank::parse_line("987654321111111"),
            Bank {
                batteries: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]
            }
        );

        assert_eq!(
            Bank::parse_line("811111111111119"),
            Bank {
                batteries: vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9]
            }
        );

        assert_eq!(
            Bank::parse_line("234234234234278"),
            Bank {
                batteries: vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]
            }
        );

        assert_eq!(
            Bank::parse_line("818181911112111"),
            Bank {
                batteries: vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]
            }
        );

        assert_eq!(
            Bank::parse_line("818"),
            Bank {
                batteries: vec![8,1,8]
            }
        );
    }
}