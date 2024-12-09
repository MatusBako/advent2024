use std::cmp::Ordering;
use std::io::{self};
use std::io::{BufRead, BufReader};
use std::fs::{exists, File};
use std::iter::zip;
use std::collections::HashMap;

struct Input
{
    lines: Vec<String>,
    parsed: Vec<Vec<i32>>,
}

fn get_input(file_path: &str) -> Result<Input, io::Error>
{
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
        .map(|(x, y): (&i32, &i32)| match x.cmp(y)
            {
                // dereferencing a borrow??
                Ordering::Greater => match x.abs_diff(*y) 
                    {
                        1 ..= 3 => Ordering::Greater,
                        _ => Ordering::Equal,
                    },
                Ordering::Less => match x.abs_diff(*y) 
                    {
                        1 ..= 3 => Ordering::Less,
                        _ => Ordering::Equal,
                    },
                Ordering::Equal => Ordering::Equal
            }
    ).collect::<Vec<Ordering>>();
    // TODO: optimize to pass vector only once with Optional<bool> (for 3 values)
    return diff.iter().all(|x: &Ordering| x == &Ordering::Less) 
        || diff.iter().all(|x: &Ordering| x == &Ordering::Greater);
}

fn signum(n: i32) -> i32
{
    match n
    {
        x if x < 0 => -1,
        x if x > 0 => 1,
        _ => 0,
    }
}

fn levels_safe_bonus(levels: &Vec<i32>) -> bool
{
    // compute 2nd order derivations and detect where it changes
    // not the memory optimal solution, but I thought it would be interesting

    let diff: Vec<i32> = zip(&levels[0..levels.len()-1],&levels[1..])
        .map(|(x, y): (&i32, &i32)| signum(x - y)).collect();
    let diff_2nd = zip(&diff[0..diff.len()-1],&diff[1..])
        .map(|(x, y): (&i32, &i32)| signum(x - y)).collect::<Vec<i32>>();

    // can't fold it like this, I need mutable hashmap defined beforehand
    // let (_, indices): (i32, HashMap<i32, Vec<i32>>) = diff_2n
    //     .fold((0, HashMap::new()), |(idx, map), x| 
    //         {
    //             map.entry(x)
    //             .and_modify(|v: &mut Vec<i32>| v.push(idx))
    //             .or_insert(vec![idx]);
    //             return (idx + 1, map)
    //         }
    //         );

    let mut indices: HashMap<i32, Vec<i32>> = HashMap::new();
    
    let _ = diff_2nd.iter()
        .enumerate()
        .map(|(idx, value): (usize, &i32)|
            match value
        {
            -1 | 1 => { 
                // TODO: why deref?
                indices.entry(*value)
                    .and_modify(|v: &mut Vec<i32>| v.push(idx as i32))
                    .or_insert(vec![idx as i32]); }, 
            _ => (),
        }).collect::<()>();

    match (indices.get(&-1), indices.get(&1))
    {
        // ascending or descending only, check ranges 1-3
        (None, None) => { return levels_safe(levels)},

        // one change only, but not saying how many elements are equal/
        (None, Some(v)) | (Some(v), None) => 
        {
            // figure out whether index subtraction is correct
            // -1 to point to the last element, -2 because 2nd diff array is shorter
            if v.len() == 1 && (v[0] == 0 || v[0] == levels.len() as i32 - 1 - 2)
            {
                // in case of difference in 0 index, we don't know which number to omit, try all 3
                // otherwise, omit last checked element, idx + 2
                if v[0] == 0
                {
                    let (mut v1, mut v2, mut v3) = (levels.clone(), levels.clone(), levels.clone());
                    v1.remove(0);
                    v2.remove(1);
                    v3.remove(2);

                    // when change is discovered on first index, we don't know
                    // which of first three numbers is problematic, so try them all
                    return levels_safe(&v1) || levels_safe(&v2) || levels_safe(&v3);
                }

                let mut copy = levels.clone();
                copy.remove((v[0] + 2) as usize);
                return levels_safe(&copy);
            }
            return false;
        },

        (Some(v1), Some(v2)) =>
        {
            if v1.len() == 1 && v2.len() == 1
            { 
                let mut idx = if v1.get(0) < v2.get(0) 
                { *v1.get(0).unwrap() } 
                else { *v2.get(0).unwrap() };

                // if first idx is problematic, we don't know which of 3 numbers should be removed
                if idx == 0
                {
                    let (mut v1, mut v2, mut v3) = (levels.clone(), levels.clone(), levels.clone());
                    v1.remove(0);
                    v2.remove(1);
                    v3.remove(2);

                    // when change is discovered on first index, we don't know
                    // which of first three numbers is problematic, so try them all
                    return levels_safe(&v1) || levels_safe(&v2) || levels_safe(&v3);

                }
                idx += 2; // increase idx by two to remove the last element

                // why dereference?
                let mut shortened = levels.clone();
                shortened.remove((idx) as usize);
                return levels_safe(&shortened);
            }
            return false; // more than one rising or falling index, fail
        }
        _ => { return levels_safe(levels); }
    }

    // first two elements determine slope (rising/falling), third is checked
    // what if first or second is wrong?

    // 0 1 2 3 3 2
    // 1 1 1 0 -1
    // 0 0 -1 -1

    // 1 2 3 3 3
    // 1 1 0 0 0
    // 0 -1 0 0

    // 2 1 2 4  6 5 7
    //-1 1 1 1 -1 1
    // 1 0 0 -1  1

    // 1  2 2 3 4
    // 1  0 1 1 
    // -1 1 0 0

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

    // bonus
    // println!("Should be safe, is safe: {safe}", safe=levels_safe_bonus(&vec![1, 2, 3, 3, 4]));
    // println!("Should be safe, is safe: {safe}", safe=levels_safe_bonus(&vec![2, 1, 2, 3, 4]));
    // println!("Should be safe, is safe: {safe}", safe=levels_safe_bonus(&vec![1, 2, 3, 4, 3]));
    // println!("Should be safe, is safe: {safe}", safe=levels_safe_bonus(&vec![8,6,4,4,1]));
    // println!("Should be safe, is safe: {safe}", safe=levels_safe_bonus(&vec![2,1,4,5]));
    // println!("Should be safe, is safe: {safe}", safe=levels_safe_bonus(&vec![7,5,3,3]));
    // println!("Should be safe, is safe: {safe}", safe=levels_safe_bonus(&vec![7,10,8,10,11]));
    // println!("Should be unsafe, is safe: {safe}", safe=levels_safe_bonus(&vec![1, 2, 3, 3, 3, 4]));
    // println!("Should be unsafe, is safe: {safe}", safe=levels_safe_bonus(&vec![1, 2, 3, 2, 1]));
    
    // return Ok(());

    let input: Input = get_input(r"D:\src\Advent2024\inputs\02.txt")?;
    // let func = levels_safe;
    let func = levels_safe_bonus;

    let _printed = zip(&input.lines, &input.parsed)
        .map(|(line, vec)| format!("{}: {}", line, func(&vec)))
        .fold("".to_owned(), |x, y| x + ": " + &y + "\n");
    println!("{_printed}");

    let _result: i32 = (&input.parsed).iter()
        //.map(|x| i32::from(levels_safe(x))).sum();
        .map(func).filter(|&x| x).count() as i32;

    println!("Result: {_result}");

    return Ok(());
}

#[cfg(test)]
mod tests 
{
    use super::*;

    // TODO: for the love of god, find a way to parametrize tests
    #[test]
    fn eq_begin_rising() 
    {
        assert_eq!(levels_safe_bonus(&vec![1,1,2,3]), true);
    }
    
    #[test]
    fn eq_begin_falling() 
    {
        assert_eq!(levels_safe_bonus(&vec![3,3,2,1]), true);
    }

    #[test]
    fn eq_end_rising() 
    {
        assert_eq!(levels_safe_bonus(&vec![1,2,3,3]), true);
    }

    #[test]
    fn eq_end_falling() 
    {
        assert_eq!(levels_safe_bonus(&vec![5,4,3,3]), true);
    }

    #[test]
    fn eq_mid_rising() 
    {
        assert_eq!(levels_safe_bonus(&vec![1,2,3,3,4]), true);
    }

    #[test]
    fn eq_mid_falling() 
    {
        assert_eq!(levels_safe_bonus(&vec![5,4,3,3,2,1]), true);
    }

    #[test]
    fn fail1() 
    {
        assert_eq!(levels_safe_bonus(&vec![1,2,5,4,3,2,1]), false);
    }

    #[test]
    fn multiple_equal() 
    {
        assert_eq!(levels_safe_bonus(&vec![1,2,2,2,2,3,4]), false);
    }

    #[test]
    fn fail2() 
    {
        assert_eq!(levels_safe_bonus(&vec![2,1,4,5,3]), false);
    }

    #[test]
    fn fail3() 
    {
        assert_eq!(levels_safe_bonus(&vec![9,7,5,3,3,3]), false);
    }

    #[test]
    fn fail4() 
    {
        assert_eq!(levels_safe_bonus(&vec![10, 8,4,5,9]), false);
    }

    #[test]
    fn eq_many() 
    {
        assert_eq!(levels_safe_bonus(&vec![1,1,1,1,1,2]), false);
    }

    #[test]
    fn second_invalid() 
    {
        assert_eq!(levels_safe_bonus(&vec![7,10,8,10,11]), true);
    }

    #[test]
    fn fail123() 
    {
        assert_eq!(levels_safe_bonus(&vec![0, 4,5,3,2,1]), false);
    }
}
