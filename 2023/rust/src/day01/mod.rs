use crate::utils::read_lines;
pub fn main() {
    let input_file_path = std::env::current_dir().unwrap()
        .join("src").join("day01").join("input.txt");
    if let Ok(lines) = read_lines(input_file_path) {
        let lines_vec:Vec<String> = lines.filter_map(|line|line.ok()).collect();
        part_a(&lines_vec);
        part_b(lines_vec);
        // println!("{}",test(lines_vec).unwrap());
    };
}

fn part_a(lines: &Vec<String>) {
    let mut number_res: i32 = 0;
    for line in lines {
        number_res += find_first_and_last_numbers_combine(&line);
    }

    println!("Day01 A result: {}", number_res)
}


fn find_first_and_last_numbers_combine(input: &String) -> i32 {
    let mut current_numbers = String::new();
    let mut res_number: i32 = 0; 
    for c in input.chars() {
        if c.is_digit(10) {
            current_numbers.push(c);
        }
    }
    if current_numbers.len() > 2 {
        let mut temp_numbers: String = String::new();
        if let Some(first_char) = current_numbers.chars().next() {
            temp_numbers.push(first_char);
        }
        if let Some(last_char) = current_numbers.chars().next_back() {
            temp_numbers.push(last_char);
        }

        res_number += &temp_numbers.parse().unwrap_or(0)
    }
    else if current_numbers.len() == 1 {
        let temp = current_numbers.to_string() + &current_numbers;
        res_number += &temp.parse().unwrap_or(0)
    }
    else if current_numbers.len() == 2 {
        res_number += &current_numbers.parse().unwrap_or(0)
    }
    else {
        print!("No number in string!");
    }

    return res_number;
}

fn part_b (input: Vec<String>) {
    let res: usize = input
        .into_iter()
        .map(|row| {
            let digits: Vec<u32> = row
                .chars()
                .enumerate()
                .into_iter()
                .map(|(i, c)| {
                    if c.is_digit(10) {
                        return Some(c.to_digit(10).unwrap());
                    }
                    let chars = row.chars().skip(i).collect::<String>();
                    if chars.starts_with("one") {
                        return Some(1);
                    } else if chars.starts_with("two") {
                        return Some(2);
                    } else if chars.starts_with("three") {
                        return Some(3);
                    } else if chars.starts_with("four") {
                        return Some(4);
                    } else if chars.starts_with("five") {
                        return Some(5);
                    } else if chars.starts_with("six") {
                        return Some(6);
                    } else if chars.starts_with("seven") {
                        return Some(7);
                    } else if chars.starts_with("eight") {
                        return Some(8);
                    } else if chars.starts_with("nine") {
                        return Some(9);
                    } else {
                        return None;
                    };
                })
                .filter(|o| match o {
                    Some(_val) => true,
                    None => false,
                })
                .map(|val| val.unwrap())
                .collect::<Vec<u32>>();
            let first = digits.first().unwrap_or(&0).to_owned();
            let last = digits.last().unwrap_or(&0).to_owned();

            format!("{}{}", first, last).as_str().parse().unwrap_or(0)
        })
        .sum();

        println!("Day01 B result: {}", res)
}