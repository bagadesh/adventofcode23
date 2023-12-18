use std::fs;
use std::io::Error;

fn main() -> Result<(), Error>{
    let collabration_doc = fs::read_to_string("input.txt")?;

    let sum: u32 = extract_digits(&collabration_doc)
        .into_iter()
        .fold(0, |acc,(f,s) | acc + (f*10)+ s);
    
    println!("sum {}", sum);
    Ok(())
}

fn extract_digits(input: &str) -> Vec<(u32,u32)> {
    return  input.split("\n").map(|item| {
        let first_digit = extract_first_digit(&item);
        let second_digit = extract_second_digit(&item);

        (first_digit, second_digit)
    }).collect();
}

fn extract_first_digit(input: &str) -> u32 {
    let mut first_digit = 0;
    for (i, c) in input.char_indices() {
        if c.is_digit(10) {
            first_digit = c.to_digit(10).unwrap();
            break;
        } 
        
        for j in 0..(i) {
            if let Some(digit) = is_number(&input[j..(i+1)]) {
                return digit;
            }
        }
    }
    return first_digit

}


fn extract_second_digit(input: &str) -> u32 {
    let mut result_digit = 0;
    for (i, c) in input.char_indices().rev() {
        if c.is_digit(10) {
            result_digit = c.to_digit(10).unwrap();
            break;
        } 
        
        for j in (i+1)..input.len() {
            if let Some(digit) = is_number(&input[i..(j+1)]) {
                return digit;
            }
        }
    }
    return result_digit 

}

fn is_number(input: &str) -> Option<u32> {
    let value = match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => return None
    };

    return Some(value)
}
