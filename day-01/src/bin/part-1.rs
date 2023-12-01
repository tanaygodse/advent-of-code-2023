fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(s: &str) -> usize {
    let mut sum: usize = 0;

    for line in s.lines() {
        let mut l: char = '0';
        let mut r: char = '0';

        for c in line.chars() {
            if c.is_numeric() {
                l = c;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_numeric() {
                r = c;
                break;
            }
        }

        let my_string: String = l.to_string() + &r.to_string();
        match my_string.parse::<usize>() {
            Ok(parsed_value) => {
                println!("Successfully parsed: {}", parsed_value);
                sum += parsed_value;
            }
            Err(e) => {
                println!("Error parsing string: {}", e);
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let text = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(text);
        assert_eq!(result, 142); // Adjust the expected result based on your test case
    }
}

