use std::collections::HashMap;

fn FindDuplicate(firstHalf: &String, secondHalf: &String) -> char {
    let mut duplicate: char = '1';
    for letter in secondHalf.chars() {
        if (firstHalf.contains(letter)) {
            duplicate = letter;
        }
    }
    duplicate
}

fn GetPriority(duplicate_letter: &char) -> u32 {
    let priority = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);
    priority[duplicate_letter]
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut Result: u32 = 0;
    for line in input.lines() {
        let SplitStrings: (&str, &str) = line.split_at(line.len() / 2);
        let firstHalf: String = SplitStrings.0.to_string();
        let secondtHalf: String = SplitStrings.0.to_string();

        let duplicateLetter: char = FindDuplicate(&firstHalf, &secondtHalf);
        Result += GetPriority(&duplicateLetter);
    }
    Some(Result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
