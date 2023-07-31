use std::collections::HashMap;

fn PointsFromResult(Shapes: Vec<&str>) -> u32 {
    let rounds = HashMap::from([
        ("AX".to_string(), 3),
        ("AY".to_string(), 6),
        ("AZ".to_string(), 0),
        ("BX".to_string(), 0),
        ("BY".to_string(), 3),
        ("BZ".to_string(), 6),
        ("CX".to_string(), 6),
        ("CY".to_string(), 0),
        ("CZ".to_string(), 3),
    ]);

    let ThisRound: String = Shapes.join("");
    rounds[&ThisRound]
}

fn PointsFromShape(Shape: &str) -> u32 {
    let mut result: u32 = 0;
    match Shape {
        "X" => result = 1,
        "Y" => result = 2,
        "Z" => result = 3,
        "A" => result = 1,
        "B" => result = 2,
        "C" => result = 3,
        _ => print!("Not valid Shape"),
    }
    result
}

fn PointsFromExpectedResult(Shapes: Vec<&str>) -> u32 {
    let rounds = HashMap::from([
        ("AX".to_string(), 3),
        ("AY".to_string(), 4),
        ("AZ".to_string(), 8),
        ("BX".to_string(), 1),
        ("BY".to_string(), 5),
        ("BZ".to_string(), 9),
        ("CX".to_string(), 2),
        ("CY".to_string(), 6),
        ("CZ".to_string(), 7),
    ]);

    let ThisRound: String = Shapes.join("");
    rounds[&ThisRound]
}

pub fn parse_input1(input: &str, score: &mut u32) {
    for line in input.lines() {
        let mut RoundScore: u32 = 0;
        if line.is_empty() {
        } else {
            let Shapes: Vec<&str> = line.split(' ').collect();
            RoundScore += PointsFromShape(Shapes[1]);
            RoundScore += PointsFromResult(Shapes);
        }
        *score += RoundScore;
    }
}

pub fn parse_input2(input: &str, score: &mut u32) {
    for line in input.lines() {
        let mut RoundScore: u32 = 0;
        if line.is_empty() {
        } else {
            let Shapes: Vec<&str> = line.split(' ').collect();
            RoundScore += PointsFromExpectedResult(Shapes);
        }
        *score += RoundScore;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    parse_input1(input, &mut result);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    parse_input2(input, &mut result);
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
