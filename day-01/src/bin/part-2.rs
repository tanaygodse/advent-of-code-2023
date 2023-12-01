use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(s: &str) -> usize {
    let mut sum: usize = 0;

    let my_dict: HashMap<&str, &str> = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "thr3e"),
        ("four", "fo4r"),
        ("five", "fi5e"),
        ("six", "s6x"),
        ("seven", "sev7n"),
        ("eight", "ei8ht"),
        ("nine", "n9ne"),
        ("zero", "ze0o"),
    ].iter().cloned().collect();
    
    let mut s1 = s.to_string();

    for (key, value) in my_dict{
        s1 = s1.replace(key, value);
    }

    for line in s1.lines() {
        println!("s: {}", line);
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
        let text = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part1(text);
        assert_eq!(result, 281); // Adjust the expected result based on your test case
        // 29 + 83 + 13  + 24 + 42 + 14 + 76
    }
}

