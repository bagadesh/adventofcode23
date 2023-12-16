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
        let mut first_digit = 0;
        let mut second_digit = 0;
        for l in item.chars() {
            if l.is_digit(10) {
                first_digit = l.to_digit(10).unwrap();
                break;
            } 
        }  
        for l in item.chars().rev() {
            if l.is_digit(10) {
                second_digit = l.to_digit(10).unwrap();
                break;
            } 
        }  
        (first_digit, second_digit)
    }).collect();
}
