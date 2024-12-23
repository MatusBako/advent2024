use std::{fs, io::{self, BufRead, Read}};
use regex::Regex;

fn get_input(file_path: &str) -> Result<String, io::Error>
{

    match fs::exists(file_path)
    {
        Ok(false) => panic!(),
        Err(e) => panic!("File not found: {e:?}"),
        _ => ()        
    }

    let file = fs::File::open(file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut result: String = String::new();
    reader.read_to_string( &mut result)?;
    
    return Ok(result);
}

fn construct_common_regex(line_length: i32) -> String
//  approach using one giant regex and or-ing occurences
// doesn't work because this doesn't cover overlapping occurences
// and different occurences can even start on the same position
{
    let partial_regexes = vec!["XMAS".to_string(), "SAMX".to_string(),
    // vertical
    format!("X.{{{l}}}M.{{{l}}}A.{{{l}}}S", l=line_length),
    format!("S.{{{l}}}A.{{{l}}}M.{{{l}}}X", l=line_length),
    // top left, bottom right
    format!("X.{{{l}}}M.{{{l1}}}A.{{{l2}}}S", l=line_length + 1, l1=line_length + 2, 
        l2=line_length + 3),
    format!("S.{{{l}}}A.{{{l1}}}M.{{{l2}}}X", l=line_length + 1, l1=line_length + 2, 
        l2=line_length + 3),
    // top right, bottom left
    format!("X.{{{l}}}M.{{{l1}}}A.{{{l2}}}S", l=line_length - 1, l1=line_length - 2, 
        l2=line_length - 3),
    format!("S.{{{l}}}A.{{{l1}}}M.{{{l2}}}X", l=line_length - 1, l1=line_length - 2, 
        l2=line_length - 3)
    ];

    let mut result = String::new();
    result.reserve(partial_regexes.iter().map(|x| x.len()).sum());

    // (?ms) enables multiline  and matching \n with .
    // TODO: why does right operand require a borrow operator here?
    return partial_regexes.iter().fold(result, | acc: String, i: &String|
        match acc
        {
            s if s.is_empty() => "(?ms)".to_string() + i, // first iterated member
            _ => acc + "|" + i,
        });
}

fn construct_regex(input: &String) -> Regex
{
    // when last command has no semicolon, it is returned??
    match Regex::new(format!("(?ms){}", input).as_str())
    {
        Ok(x) => x,
        Err(e) => panic!("Could not create regex \"{}\": {}", input, e)
    }
}

fn construct_regex_base(line_length: i32) -> Vec<regex::Regex>
// use different regex for each pattern
{
    let partial_regexes: Vec<String> = vec![
        "XMAS".to_string(), "SAMX".to_string(),
        // vertical
        format!("X.{{{l}}}M.{{{l}}}A.{{{l}}}S", l=line_length),
        format!("S.{{{l}}}A.{{{l}}}M.{{{l}}}X", l=line_length),
        // top left, bottom right
        format!("X.{{{l}}}M.{{{l}}}A.{{{l}}}S", l=line_length + 1),
        format!("S.{{{l}}}A.{{{l}}}M.{{{l}}}X", l=line_length + 1),
        // top right, bottom left
        format!("X.{{{l}}}M.{{{l}}}A.{{{l}}}S", l=line_length - 1),
        format!("S.{{{l}}}A.{{{l}}}M.{{{l}}}X", l=line_length - 1),
    ];

    return partial_regexes.iter()
        .map(construct_regex)
        .collect();
}

fn construct_regex_bonus(line_length: i32) -> Vec<regex::Regex>
{
    let partial_regexes: Vec<String> = vec![
        format!("M.M.{{{l1}}}A.{{{l2}}}S.S", l1=line_length - 1, l2=line_length - 1),
        format!("M.S.{{{l1}}}A.{{{l2}}}M.S", l1=line_length - 1, l2=line_length - 1),
        format!("M.S.{{{l1}}}A.{{{l2}}}S.M", l1=line_length - 1, l2=line_length - 1),
        format!("S.M.{{{l1}}}A.{{{l2}}}M.S", l1=line_length - 1, l2=line_length - 1),
        format!("S.M.{{{l1}}}A.{{{l2}}}S.M", l1=line_length - 1, l2=line_length - 1),
        format!("S.S.{{{l1}}}A.{{{l2}}}M.M", l1=line_length - 1, l2=line_length - 1),
    ];

    return partial_regexes.iter()
        .map(construct_regex)
        .collect();
}

fn get_line_length(input: &String) -> i32
{
    let mut idx: i32 = 0;
    input.chars().find(|&x| 
        match x == '\n'
        {
            true => { return true; },
            false => { idx += 1; return false; } ,
        });
    return idx;
}

fn find_regexes(input: &String, regexes: &Vec<Regex>, line_length: i32) -> Result<i32, Box<dyn std::error::Error>>
{
    let mut count = 0;
    let mut idx = 0;

    // doesn't work because overlapping regexes won't be found
    // for capture in re.captures_iter(&input)
    // {
    //     count += 1
    //     println!("{:?}",capture.get(0))
    // }

    // 
    // still doesn't work, won't find different occurences starting on the same char
    // while true
    // {
    //     match re.find_at(&input, idx)
    //     {
    //         Some(found) => 
    //         {
    //             idx = found.start() + 1;
    //             count += 1;
    //         }
    //         None => { break; }
    //     }
    // }

    // way to go, iterate over all regexes one at a time
    for reg in regexes
    {
        idx = 0;
        //println!("Regex: {}", reg);

        loop
        {
            match reg.find_at(&input, idx)
            {
                Some(found) => 
                {
                    idx = found.start() + 1;
                    count += 1;
                    // println!("Line {}, char {}: {}", 
                    //     (found.start() as i32) / line_length,
                    //     (found.start() as i32) % line_length,
                    //     found.as_str());//.replace("\n", "|")
                }
                None => { break; }
            }
        }
    }
    
    return Ok(count);
}

fn find_xmas(input: &String) -> Result<i32, Box<dyn std::error::Error>>
{
    let line_length = get_line_length(&input);
    println!("Line length: {}", &line_length);

    let regexes = construct_regex_base(line_length);
    return find_regexes(input, &regexes, line_length);
}

fn find_xmas_bonus(input: &String) -> Result<i32, Box<dyn std::error::Error>>
{
    let line_length = get_line_length(&input);
    println!("Line length: {}", &line_length);

    let regexes = construct_regex_bonus(line_length);
    return find_regexes(input, &regexes, line_length);
}

fn main()
{
    let file_path: &str = r"D:\src\Advent2024\inputs\042.txt";

    let input: String = match get_input(file_path)
    {
        Ok(s) => s,
        Err(e) => panic!("File not found: {e:?}")
    };

    // match find_xmas(input)
    match find_xmas_bonus(&input)
    // match find_xmas("Sawd\naAaw\nfeMg\nawdX\n".to_string())
    // match find_xmas("awdS\naaAw\nfMeg\nXawd\n".to_string()) // diagonal tr bl
    // match find_xmas("Saaa\nAaaa\nMaaa\nXaaa\n".to_string()) // vertical
    // match find_xmas_bonus("MaMa\nwAaa\nSaSa\nwaaa\n".to_string()) // vertical
    {
        Ok(i) => println!("Found {} occurences", i),
        Err(e) => panic!("Search for xmas failed: {e:?}")
    };

}

#[cfg(test)]
mod tests 
{
    use super::*;

    // TODO: for the love of god, find a way to parametrize tests
    #[test]
    fn horizontal() 
    {
        let result: i32 = find_xmas(&"XMAS".to_string()).unwrap_or(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn horizontal_bw() 
    {
        let result: i32 = find_xmas(&"SAMX".to_string()).unwrap_or(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn vertical() 
    {
        let result: i32 = find_xmas(&"Xawd\nMwaw\nAddg\nSawd\n".to_string()).unwrap_or(0);
        assert_eq!(result, 1);
    }
    
    #[test]
    fn vertical_bw() 
    {
        let result: i32 = find_xmas(&"Sawd\nAwaw\nMfeg\nXawd\n".to_string()).unwrap_or(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn tlbr() 
    {
        let result: i32 = find_xmas(&"Xawd\naMaw\nfeAg\nawdS\n".to_string()).unwrap_or(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn tlbr_bw()
    {
        let result: i32 = find_xmas(&"Sawd\naAaw\nfeMg\nawdX\n".to_string()).unwrap_or(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn trbl()
    {
        let result: i32 = find_xmas(&"awdX\naaMw\nfAeg\nSawd\n".to_string()).unwrap_or(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn trbl_bw()
    {
        let result: i32 = find_xmas(&"awdS\naaAw\nfMeg\nXawd\n".to_string()).unwrap_or(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn bonus()
    {
        let inputs = vec![
            "MaMa\nwAaa\nSaSa\nwaaa\n".to_string(),
            "MaSa\nwAaa\nMaSa\nwaaa\n".to_string(),
            "MaSa\nwAaa\nSaMa\nwaaa\n".to_string(),
            "SaMa\nwAaa\nMaSa\nwaaa\n".to_string(),
            "SaMa\nwAaa\nSaMa\nwaaa\n".to_string(),
            "SaSa\nwAaa\nMaMa\nwaaa\n".to_string(),
        ];

        assert_eq!(
            inputs.iter()
                .map(|input: &String| -> i32
                    {    match find_xmas_bonus(input)
                        {
                            Ok(result) => result,
                            Err(e) => panic!("{e}"),
                        }    
                    })
                .sum::<i32>(), 
            inputs.len() as i32);
    }
}