struct Results {
    top: u32,
    second: u32,
    third: u32,
}

impl Results {
    fn update(&mut self, calories: u32) {
        if calories > self.top {
            self.third = self.second;
            self.second = self.top;
            self.top = calories;
        } else if calories > self.second {
            self.third = self.second;
            self.second = calories;
        } else if calories > self.third {
            self.third = calories;
        }
    }

    fn sum(self) -> u32 {
        let total: u32 = self.top + self.second + self.third;
        return total;
    }
}

fn parse_input(input: &str, result: &mut Results) {
    let mut calories: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            result.update(calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }
    result.update(calories);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: Results = Results {
        top: 0,
        second: 0,
        third: 0,
    };
    parse_input(input, &mut result);
    Some(result.top)
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut result: Results = Results {
        top: 0,
        second: 0,
        third: 0,
    };
    parse_input(_input, &mut result);
    Some(result.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
