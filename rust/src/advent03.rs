use std::error::Error;
use std::fs;
use std::io::{self, Read};
use regex::{self, Regex};

fn get_input(file_path: &str) -> Result<String, io::Error>
{
    match fs::exists(file_path)
    {
        Ok(false) => panic!("Path does not exist: {file_path}"),
        Err(e) => panic!("Error: {e:?}"),
        _ => (),
    }

    let file = fs::File::open(file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut result: String = String::new();
    
    // TODO: &mut ??? mutable borrow
    return match reader.read_to_string(&mut result)
    {
        // mut String -> String
        Ok(_) => Ok(result.to_string()),
        Err(e) => Err(e)
    };
}

fn compute(input: &str) -> Result<i32, Box<dyn Error>>
{
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)")?;
    let mut result: i32 = 0;

    for captures in re.captures_iter(&input)
    {
        result += compute_capture(&captures["x"], &captures["y"])?;
    }

    return Ok(result);
}

fn compute_bonus(input: &str) -> Result<i32, Box<dyn Error>>
{
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)|do\(\)|don't\(\)")?;
    let mut result: i32 = 0;

    let mut enabled: bool= true;

    for capture in re.captures_iter(&input)
    {
        if capture[0].starts_with("mul") && enabled
        { 
            result += compute_capture(&capture["x"], &capture["y"])?; 
        }
        else if capture[0].starts_with("don't()")
        {
            enabled = false;     
        }
        else if capture[0].starts_with("do()")
        {
            enabled = true;
        }
    }

    return Ok(result);
}

fn compute_capture(x: &str, y: &str) -> Result<i32, Box<dyn Error>>
{
    let x: i32 = x.parse::<i32>()?;
    let y: i32 = y.parse::<i32>()?;
    let capture_result = x * y;
    //println!("{x}, {y}: {capture_result}");
    return Ok(capture_result);

}

fn _test_regex() -> Result<(), Box<dyn std::error::Error>>
{
    // let re = Regex::new(r"(?<x>\d{1,3})")?;
    // let string = "12|123|1234";

    let re = Regex::new(r"(?<x>don't\(\))")?;
    let string = "do()|123|don't()";

    println!("{:?}", re.captures(string));
    return Ok(());
}

fn main() -> Result<(), Box<dyn Error>>
{
    let file_path = r"D:\src\Advent2024\inputs\03.txt";
    let input = get_input(file_path)?;

    //_test_regex()?;

    // let str = "mul(2,3)awdafawmmul[]mul9mul(4,1)".to_string();
    let result = compute(&input)?;
    println!("Result: {result}");
    let bonus_result = compute_bonus(&input)?;
    println!("Bonus result: {bonus_result}");

    return Ok(());
}