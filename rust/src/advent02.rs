use std::cmp::Ordering;
use std::io::{self};
use std::io::{BufRead, BufReader};
use std::fs::{exists, File};
use std::iter::zip;

struct Input
{
    lines: Vec<String>,
    parsed: Vec<Vec<i32>>,
}

fn get_input() -> Result<Input, io::Error>
{
    let file_path = r"D:\src\Advent2025\inputs\022.txt";

    if exists(file_path).is_err()
    {
        panic!("File not found: {file_path}");
    }

    // otaznicek prakticky robi unwrap, odbaluje
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|x|x.unwrap()).collect();
    let parsed: Vec<Vec<i32>> = lines.clone().iter().map(parse_line).collect();
    return Ok(Input { lines: lines, parsed: parsed });
}

// // why does this needs a result? -> lines() returns a result for some reason
// fn parse_result_line(line: Result<String, io::Error>) -> Vec<i32>
// {
//     return match line 
//     {
//         Ok(string) => parse_line(&string),
//         Err(e) => panic!("Error while reading line: {e}")
//     };
// }

fn parse_line(line: &String) -> Vec<i32>
{
    return line.split_whitespace()
        .map(|x| match x.parse::<i32>()
            {
                Ok(n) => n,
                Err(e) => panic!("Error while parsing \"{c}\": {e}", c=x)
            }).collect();
}

fn levels_safe(levels: &Vec<i32>) -> bool
{
    // TODO: why &? why do I need to borrow when slicing?
    // TODO: why does this needs to be a mutable if all() is not changing it?
    let diff = zip(&levels[0..levels.len()-1],&levels[1..])
    //.map(|(x, y): (&i32, &i32)| x - y > 0)
        .map(|(x, y): (&i32, &i32)| x.cmp(y)).collect::<Vec<Ordering>>();
    // TODO: optimize to pass vector only once with Optional<bool> (for 3 values)
    return diff.iter().all(|x: &Ordering| x == &Ordering::Less) 
        || diff.iter().all(|x: &Ordering| x == &Ordering::Greater);
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    //println!("{arr:?}", arr=&v1[..v1.len()-1]);
    // println!("Should be safe and is: {t}", t=levels_safe(&vec![1,2,3,4,5,6]));
    // println!("Should be safe and is: {t}", t=levels_safe(&vec![4,3,2,1]));
    // println!("Should be safe and is: {t}", t=levels_safe(&vec![8,9,10,11]));
    // println!("Should be unsafe, is safe: {t}", t=levels_safe(&vec![4,6,2,3]));
    // println!("Should be unsafe, is safe: {t}", t=levels_safe(&vec![1,1,1,1]));
    // println!("Should be unsafe, is safe: {t}", t=levels_safe(&vec![8,6,7,3]));
    // println!("Should be unsafe, is safe: {safe}", safe=levels_safe(&vec![11, 22, 22, 33]));
    // println!("Should be unsafe, is safe: {safe}", safe=levels_safe(&vec![11, 9, 9, 8]));
    // println!("Should be unsafe, is safe: {safe}", safe=levels_safe(&vec![2, 2, 1, 2, 2]));
    // println!("Should be unsafe, is safe: {safe}", safe=levels_safe(&vec![2, 2, 3, 4, 4]));
    // println!("Should be unsafe, is safe: {safe}", safe=levels_safe(&vec![7, 10, 8, 10, 11]));
    // println!("Should be unsafe, is safe: {safe}", safe=levels_safe(&vec![29, 28, 27, 25, 26, 25, 22, 20]));
    // println!("Should be unsafe, is safe: {safe}", safe=levels_safe(&vec![75, 77, 72, 70, 69]));
    
    let input: Input = get_input()?;

    let _printed = zip(&input.lines, &input.parsed)
        .map(|(line, vec)| format!("{}: {}", line, levels_safe(&vec)))
        .fold("".to_owned(), |x, y| x + ": " + &y + "\n");
    println!("{_printed}");
    let _result: i32 = (&input.parsed).iter()
        //.map(|x| i32::from(levels_safe(x))).sum();
        .map(levels_safe).filter(|&x| x).count() as i32;

    println!("Result: {_result}");

    //println!("Should be unsafe, is safe: {safe}", safe=levels_safe(&vec![36, 40, 38]));
    return Ok(());
}