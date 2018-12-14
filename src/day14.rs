use aoc::{Result};

fn main() -> Result<()> {
    let s = aoc::read_input()?;

    let nr_recipes = s.parse::<u32>()
        .map_err::<Box<std::num::ParseIntError>, _>(|e| e.into())?;

    part1(nr_recipes)?;
    part2(&s)?;

    Ok(())
}

fn digits(mut val: u32) -> Vec<u32> {
    let mut digits = Vec::new();

    if val == 0 {
        digits.push(val);
        return digits;
    }

    while val > 0 {
        let digit = val % 10;
        val = val / 10;

        digits.push(digit);
    }

    digits.reverse();

    digits
}

fn next_index(cur: usize, recipes: &[u32]) -> usize {
    assert!(!recipes.is_empty());

    let val = recipes[cur];

    let forward = cur + 1 + val as usize;

    let n = forward % recipes.len();

    n
}

fn part1(nr_recipes: u32) -> Result<String> {
    let mut recipes: Vec<u32> = vec![3, 7];

    let mut elf1: usize = 0;
    let mut elf2: usize = 1;

    let mut idx =0 ;
    loop {

        if recipes.len() >= (nr_recipes as usize) + 10 || idx >= 1_000_000_000 {
            break;
        }

        let sum = recipes[elf1] + recipes[elf2];

        recipes.append(&mut digits(sum));

        elf1 = next_index(elf1, &recipes);
        elf2 = next_index(elf2, &recipes);

        idx += 1;
    }

    let next_scores: String = recipes.iter()
        .skip(nr_recipes as usize)
        .take(10)
        .map(|v| v.to_string())
        .collect();

    eprintln!("part1: {}", next_scores);

    Ok(next_scores)
}

fn part2(s: &str) -> Result<u32> {
    let mut expected_digits: Vec<u32> = s.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    eprintln!("expected_digits {:?}", expected_digits);

    // Reverse the expected since we
    // start searching from the back anyway
    expected_digits.reverse();

    let mut recipes: Vec<u32> = vec![3, 7];

    let mut elf1: usize = 0;
    let mut elf2: usize = 1;

    let mut idx = 0 ;

    let mut skip = 0;

    'outer: loop {
        let all_match = expected_digits.iter()
            .zip(recipes.iter().rev())
            .all(|(a, b)| a == b);

        if all_match || idx >= 1_000_000_000 {
            break;
        }

        let sum = recipes[elf1] + recipes[elf2];

        let mut new_digits = digits(sum);

        let new_digit_len = new_digits.len();

        recipes.append(&mut new_digits);

        // Since we added new_digit_len number of digits
        // this round, we'll start searching backwards
        // to check each added digit if they form the
        // expected sequence
        for skip_i in 0..new_digit_len {
            let all_match = expected_digits.iter()
                .zip(recipes.iter().rev().skip(skip_i))
                .all(|(a, b)| a == b);

            if all_match {
                eprintln!("skip {}", skip_i);
                skip = skip_i;
                break 'outer;
            }
        }

        elf1 = next_index(elf1, &recipes);
        elf2 = next_index(elf2, &recipes);

        idx += 1;
    }

    let len = recipes.len() - (expected_digits.len() + skip);

    eprintln!("part2: {}", len);

    Ok(len as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        assert_eq!(0, next_index(0, &[3, 7, 1, 0]));
        assert_eq!(1, next_index(1, &[3, 7, 1, 0]));
    }

    #[test]
    fn test_digits() {
        assert_eq!(vec![5, 0], digits(50));
        assert_eq!(vec![1, 0], digits(10));
        assert_eq!(vec![1], digits(1));
        assert_eq!(vec![0], digits(0));
        assert_eq!(vec![1,1,1], digits(111));
    }

    #[test]
    fn part1_example_input() {
        assert_eq!("0124515891", part1(5).unwrap());
        assert_eq!("5158916779", part1(9).unwrap());
        assert_eq!("9251071085", part1(18).unwrap());
        assert_eq!("5941429882", part1(2018).unwrap());
    }

    #[test]
    fn part2_example_input() {
        assert_eq!(9, part2("51589").unwrap());
        assert_eq!(5, part2("01245").unwrap());
        assert_eq!(18, part2("92510").unwrap());
        assert_eq!(2018, part2("59414").unwrap());
    }
}