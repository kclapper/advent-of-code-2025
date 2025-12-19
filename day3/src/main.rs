use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
struct Bank {
    batteries: Vec<u64>,
    max_enabled: u64
}

fn digits_to_value(digits: &[u64]) -> u64 {
    let mut value = 0;
    for (index, digit) in digits.iter().rev().enumerate() {
        value += digit * ((10 as u64).pow(index as u32));
    }
    return value;
}

fn shift(digits: &[u64], value: u64) -> Vec<u64> {
    let mut shift_index: usize = 0;
    for (index, digit) in digits.iter().enumerate() {
        if index == 0 {
            continue;
        }

        if digits[index - 1] < *digit {
            shift_index = index - 1;
            break;
        }
    }

    let mut shifted = Vec::with_capacity(digits.len());
    shifted.extend_from_slice(&digits[0..shift_index]);
    shifted.extend_from_slice(&digits[shift_index + 1..]);
    shifted.push(value);
    return shifted;
}

impl Bank {
    fn max_joltage(&self) -> u64 {
        let max_batteries = self.max_enabled as usize;
        let mut digits = Vec::from(&self.batteries[0..max_batteries]);
        let last_index = digits.len() - 1;

        let mut max = digits_to_value(&digits[..]);

        for current in self.batteries.iter().skip(max_batteries) {
            let shift = shift(&digits[..], *current);
            let shifted_value = digits_to_value(&shift[..]);

            if shifted_value > max {
                digits = shift;
                max = shifted_value;
            } else if *current > digits[last_index] {
                digits[last_index] = *current;
                max = digits_to_value(&digits[..]);
            }
        }
        
        return max;
    }

    fn parse_line(line: &str, max_enabled: u64) -> Bank {
        let mut values = Vec::with_capacity(64);
        for current in line.chars() {
            values.push(current.to_digit(10).unwrap() as u64);
        }

        Bank {
            batteries: values,
            max_enabled: max_enabled
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
            acc + Bank::parse_line(line, 12).max_joltage()
        });

    println!("Total joltage: {}", total_joltage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_test() {
        assert_eq!(
            shift(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 10),
            vec![8,7,6,5,4,3,2,1,1,1,1,1,1,1,10]
        );

        assert_eq!(
            shift(&[9,8], 10),
            vec![8,10]
        );
    }

    #[test]
    fn digits_to_value_test() {
        assert_eq!(
            digits_to_value(&[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]),
            987654321111111
        );

        assert_eq!(
            digits_to_value(&[9,8]),
            98
        );
    }

    #[test]
    fn max_bank_joltage() {
        let mut bank = Bank {
            batteries: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1],
            max_enabled: 2
        };
        assert_eq!(bank.max_joltage(), 98);

        bank = Bank {
            batteries: vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9],
            max_enabled: 2
        };
        assert_eq!(bank.max_joltage(), 89);

        bank = Bank {
            batteries: vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8],
            max_enabled: 2
        };
        assert_eq!(bank.max_joltage(), 78);

        bank = Bank {
            batteries: vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1],
            max_enabled: 2
        };
        assert_eq!(bank.max_joltage(), 92);

        bank = Bank {
            batteries: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1],
            max_enabled: 12
        };
        assert_eq!(bank.max_joltage(), 987654321111);

        bank = Bank {
            batteries: vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9],
            max_enabled: 12
        };
        assert_eq!(bank.max_joltage(), 811111111119);

        bank = Bank {
            batteries: vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8],
            max_enabled: 12
        };
        assert_eq!(bank.max_joltage(), 434234234278);

        bank = Bank {
            batteries: vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1],
            max_enabled: 12
        };
        assert_eq!(bank.max_joltage(), 888911112111);
    }

    #[test]
    fn parse_bank() {
        // 987654321111111 - 98
        // 811111111111119 - 89
        // 234234234234278 - 78
        // 818181911112111 - 92

        assert_eq!(
            Bank::parse_line("987654321111111", 2),
            Bank {
                batteries: vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1],
                max_enabled: 2
            }
        );

        assert_eq!(
            Bank::parse_line("811111111111119", 2),
            Bank {
                batteries: vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9],
                max_enabled: 2
            }
        );

        assert_eq!(
            Bank::parse_line("234234234234278", 2),
            Bank {
                batteries: vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8],
                max_enabled: 2
            }
        );

        assert_eq!(
            Bank::parse_line("818181911112111", 2),
            Bank {
                batteries: vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1],
                max_enabled: 2
            }
        );

        assert_eq!(
            Bank::parse_line("818", 2),
            Bank {
                batteries: vec![8,1,8],
                max_enabled: 2
            }
        );
    }
}