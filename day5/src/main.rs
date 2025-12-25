use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;
use std::cmp::Ordering;

#[derive(Debug)]
struct FreshRange(u64, u64);

impl PartialEq for FreshRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for FreshRange {}

impl PartialOrd for FreshRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let first = self.0.partial_cmp(&other.0)?;
        match first {
            Ordering::Equal => self.1.partial_cmp(&other.1),
            _ => Some(first)
        }
    }
}

impl Ord for FreshRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .cmp(&other.0)
            .then(self.1.cmp(&other.1))
    }
}

impl FreshRange {
    fn compare_ingrediant(&self, ingrediant: u64) -> Ordering {
        let FreshRange(lower, upper) = *self;

        if ingrediant < lower {
            Ordering::Less
        } else if ingrediant > upper {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    fn parse(input: &str) -> FreshRange {
        input.trim().split('-').map(|x| x.parse::<u64>().unwrap()).collect()
    }

    fn contains(&self, other: &Self) -> bool {
        self.0 <= other.0 && other.0 <= self.1
        || self.0 <= other.1 && other.1 <= self.1
    }

    fn merge(&self, other: &Self) -> FreshRange {
        FreshRange(
            min(self.0, other.0),
            max(self.1, other.1)
        )
    }
}

impl FromIterator<u64> for FreshRange {
    fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
        let mut iterator = iter.into_iter();
        let lower = iterator.next().unwrap();
        let upper = iterator.next().unwrap();
        FreshRange(lower, upper)
    }
}

fn is_fresh(ranges: &[FreshRange], item: u64) -> bool {
    if ranges.len() == 0 {
        return false;
    }

    if ranges.len() == 1 {
        return ranges[0].compare_ingrediant(item) == Ordering::Equal;
    }

    let mid = ranges.len() / 2; 
    match ranges[mid].compare_ingrediant(item) {
        Ordering::Less => is_fresh(&ranges[0..mid], item),
        Ordering::Greater => is_fresh(&ranges[mid + 1..], item),
        Ordering::Equal => true
    }
}

fn parse_ranges<'a, T: Iterator<Item = &'a str>>(lines: &mut T) -> Vec<FreshRange> {
    let mut ranges: Vec<FreshRange> = Vec::new();

    loop {
        if let Some(line) = lines.next() {
            if line.trim() == "" {
                break;
            }
            ranges.push(FreshRange::parse(line));
        } else {
            break;
        }
    } 

    if ranges.len() < 2 {
        return ranges;
    }

    ranges.sort();
    for index in (1..ranges.len()).rev() {
        let first = &ranges[index - 1];
        let second = &ranges[index];

        if first.contains(second) {
            ranges[index - 1] = first.merge(second);
            ranges.remove(index);
        }
    }
    
    return ranges;
}

fn parse_items<'a, T: Iterator<Item = &'a str>>(lines: &mut T) -> Vec<u64> {
    let mut items: Vec<u64> = Vec::new();

    loop {
        if let Some(line) = lines.next() {
            if line.trim() == "" {
                return items;
            }
            items.push(line.parse().unwrap());
        } else {
            return items;
        }
    } 
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let file = fs::read_to_string(file_name).unwrap();
    let mut lines = file.lines();

    let ranges = parse_ranges(&mut lines);
    let items = parse_items(&mut lines);

    let fresh_items = items.iter().filter(|item| {
        is_fresh(&ranges[..], **item)
    });

    print!("Fresh item count: {}\n", fresh_items.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fresh_range() {
        let mut range = FreshRange::parse("1-5");
        assert_eq!(range, FreshRange(1, 5));

        range = FreshRange::parse(" 1-5 ");
        assert_eq!(range, FreshRange(1, 5));

        range = FreshRange::parse("1-5\n");
        assert_eq!(range, FreshRange(1, 5));

        range = FreshRange::parse("100000-500000\n");
        assert_eq!(range, FreshRange(100000, 500000));
    }

    #[test]
    fn fresh_range_equality() {
        assert_eq!(FreshRange(1, 2), FreshRange(1, 2));
        assert_ne!(FreshRange(2, 2), FreshRange(1, 2));
        assert_ne!(FreshRange(1, 3), FreshRange(1, 2));
    }

    #[test]
    fn compare_fresh_range_and_ingrediant() {
        let range = FreshRange(10, 20);

        assert_eq!(range.compare_ingrediant(5), Ordering::Less);

        assert_eq!(range.compare_ingrediant(10), Ordering::Equal);
        assert_eq!(range.compare_ingrediant(11), Ordering::Equal);
        assert_eq!(range.compare_ingrediant(19), Ordering::Equal);
        assert_eq!(range.compare_ingrediant(20), Ordering::Equal);

        assert_eq!(range.compare_ingrediant(21), Ordering::Greater);
    }

    #[test]
    fn compare_fresh_ranges() {
        let range = FreshRange(10, 20);

        assert_eq!(range.cmp(&FreshRange(11, 21)), Ordering::Less);
        assert_eq!(range.cmp(&FreshRange(11, 19)), Ordering::Less);
        assert_eq!(range.cmp(&FreshRange(9, 21)), Ordering::Greater);
        assert_eq!(range.cmp(&FreshRange(9, 19)), Ordering::Greater);
        assert_eq!(range.cmp(&FreshRange(10, 19)), Ordering::Greater);
        assert_eq!(range.cmp(&FreshRange(10, 21)), Ordering::Less);
        assert_eq!(range.cmp(&FreshRange(10, 20)), Ordering::Equal);
    }

    #[test]
    fn partial_compare_fresh_ranges() {
        let range = FreshRange(10, 20);

        assert_eq!(range.partial_cmp(&FreshRange(11, 21)).unwrap(), Ordering::Less);
        assert_eq!(range.partial_cmp(&FreshRange(11, 19)).unwrap(), Ordering::Less);
        assert_eq!(range.partial_cmp(&FreshRange(9, 21)).unwrap(), Ordering::Greater);
        assert_eq!(range.partial_cmp(&FreshRange(9, 19)).unwrap(), Ordering::Greater);
        assert_eq!(range.partial_cmp(&FreshRange(10, 19)).unwrap(), Ordering::Greater);
        assert_eq!(range.partial_cmp(&FreshRange(10, 21)).unwrap(), Ordering::Less);
        assert_eq!(range.partial_cmp(&FreshRange(10, 20)).unwrap(), Ordering::Equal);
    }

    #[test]
    fn sort_fresh_range() {
        let mut ranges = vec![
            FreshRange(1, 2),
            FreshRange(100, 550),
            FreshRange(3, 4),
            FreshRange(10, 15),
            FreshRange(10, 11),
            FreshRange(10, 10),
            FreshRange(8, 110),
            FreshRange(7, 9),
            FreshRange(8, 9),
        ];
        ranges.sort();

        assert_eq!(
            ranges,
            vec![
                FreshRange(1, 2),
                FreshRange(3, 4),
                FreshRange(7, 9),
                FreshRange(8, 9),
                FreshRange(8, 110),
                FreshRange(10, 10),
                FreshRange(10, 11),
                FreshRange(10, 15),
                FreshRange(100, 550),
            ]
        );
    }

    #[test]
    fn find_item() {
        let ranges = vec![
            FreshRange(1, 4),
            FreshRange(8, 110),
            FreshRange(200, 550),
        ];

        assert!(is_fresh(&ranges, 8));
        assert!(is_fresh(&ranges, 9));
        assert!(is_fresh(&ranges, 10));
        assert!(is_fresh(&ranges, 1));
        assert!(is_fresh(&ranges, 2));
        assert!(is_fresh(&ranges, 3));
        assert!(is_fresh(&ranges, 4));
        assert!(is_fresh(&ranges, 110));
        assert!(is_fresh(&ranges, 200));
        assert!(is_fresh(&ranges, 109));

        assert!(!is_fresh(&ranges, 600));
        assert!(!is_fresh(&ranges, 5));
        assert!(!is_fresh(&ranges, 6));
    }

    #[test]
    fn parse_input_ranges() {
        let input = "99-200\n1-5\n20-30\n5-10\n50-100\n\n88";
        let mut lines = input.lines();

        assert_eq!(
            parse_ranges(&mut lines),
            vec![
                FreshRange(1, 10),
                FreshRange(20, 30),
                FreshRange(50, 200),
            ]
        );

        assert_eq!(lines.next().unwrap(), "88");
    }

    #[test]
    fn parse_input_items() {
        let input = "88\n55\n1";
        let mut lines = input.lines();

        assert_eq!(
            parse_items(&mut lines),
            vec![
                88, 
                55,
                1
            ]
        );
    }

    #[test]
    fn range_contains() {
        assert!(FreshRange(1, 10).contains(&FreshRange(2, 3)));
        assert!(FreshRange(3, 10).contains(&FreshRange(2, 3)));
        assert!(FreshRange(1, 2).contains(&FreshRange(2, 3)));
        assert!(FreshRange(3, 4).contains(&FreshRange(2, 3)));
    }

    #[test]
    fn range_merge() {
        assert_eq!(
            FreshRange(1, 10).merge(&FreshRange(2, 3)),
            FreshRange(1, 10)
        );
        assert_eq!(
            FreshRange(3, 10).merge(&FreshRange(2, 3)),
            FreshRange(2, 10)
        );
        assert_eq!(
            FreshRange(1, 2).merge(&FreshRange(2, 3)),
            FreshRange(1, 3)
        );
        assert_eq!(
            FreshRange(3, 4).merge(&FreshRange(2, 3)),
            FreshRange(2, 4)
        );
    }
}