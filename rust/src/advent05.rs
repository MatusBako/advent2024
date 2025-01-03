
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{BufRead, BufReader};
use std::fs::{exists, File};

#[derive(Debug)]
struct Input
{
    rules: Vec<(i32, i32)>,
    manuals: Vec<Vec<i32>>,
}

#[derive(Debug)]
struct Data
{
    // map page ID to sorted array of page IDs that can't be before
    // when page is added
    follows: HashMap<i32, Vec<i32>>,
    // each manual is an ordered list of pages
    manuals: Vec<Vec<i32>>,
}

fn parse_rule_line(line: &String) -> Result<(i32, i32), Box<dyn std::error::Error>>
{
    let result: Vec<i32> = line.split("|").map(|x| match x.parse::<i32>()
    {
        Ok(i) => i,
        Err(e) => panic!("Error on parsing rule \"{line}\": {e}"),
    }).collect();

    if result.len() != 2
    {
        panic!("Incorrect number of rules on line: {}", line);
    }
    return Ok((result[0], result[1]))
}

fn parse_manual_line(line: &String) -> Result<Vec<i32>, Box<dyn std::error::Error>>
{
    let parsed: Vec<i32> = line.split(",").map(|x| match x.parse::<i32>()
    {
        Ok(i) => i,
        Err(e) => panic!("Error while parsing manual: {e}")
    }).collect();
    return Ok(parsed);
}

fn get_input(file_path: &str) -> Result<Input, Box<dyn std::error::Error>>
{
    if !exists(file_path).is_ok_and(|x| x)
    {
        panic!("File not found: {}", file_path);
    }

    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut line = "".to_string();

    let mut rules: Vec<(i32, i32)> = vec![];
    let mut manuals: Vec<Vec<i32>> = vec![];

    loop 
    {
        reader.read_line(&mut line)?;
        
        // two replaces because of win/linux compatibility
        line = line.replace("\r", "").replace("\n", "");

        if line.chars().all(|c| c.is_whitespace())
        {
            break;
        }

        rules.push(parse_rule_line(&line)?);
        
        // TODO: find a better way to read line?
        line = "".to_string();
    }

    loop
    {
        reader.read_line(&mut line)?;

        line = line.replace("\r", "").replace("\n", "");
        
        if line.chars().all(|c| c.is_whitespace())
        {
            break;
        }

        manuals.push(parse_manual_line(&line)?);

        // TODO: find a better way to read line?
        line = "".to_string();
    }

    return Ok(Input{ rules: rules, manuals: manuals});
}

fn transform_input(input: &Input) -> Data
{
    let mut rules: HashMap<i32, BinaryHeap<i32>> = HashMap::new();

    input.rules.iter().for_each(|(x, y)| 
    {
        rules.entry(*x)
        .and_modify(|heap| heap.push(*y))
        .or_insert(BinaryHeap::from([*y]));
    });

    // TODO: how to do this in one command
    /*let tuples = prohibited.iter()
        .map(|(k, v): (&i32, &BinaryHeap<i32>)| 
            (*k, v.into_sorted_vec()))
        .collect::<(i32, Vec<i32>)>();*/
    let mut trasformed: HashMap<i32, Vec<i32>> = HashMap::new();

    rules.iter()
        .for_each(|(id, heap): (&i32, &BinaryHeap<i32>)|
        {
            trasformed.entry(*id)
                // TODO: ineffective clone
                .insert_entry(heap.clone().into_sorted_vec());
        });

    return Data { 
        follows: trasformed,
        manuals: input.manuals.clone() }
}

fn count_valid_manuals(data: &Data) -> i32
{
    let mut sum = 0;

    data.manuals.iter().for_each(|manual|
        {
            if is_manual_valid(manual, &data.follows)
            {
                sum += extract_middle_page(manual);
            }
        });

    return sum;
}

fn count_invalid_manuals(data: &Data) -> i32
{
    let mut sum = 0;

    data.manuals.iter().for_each(|manual|
        {
            if !is_manual_valid(manual, &data.follows)
            {
                let reordered_manual = reorder_manual(manual, &data.follows);
                sum += extract_middle_page(&reordered_manual);
            }
        });

    return sum;
}

fn reorder_manual(manual: &Vec<i32>, following: &HashMap<i32, Vec<i32>>) -> Vec<i32>
{
    /* 
    find first character that breaks a rule
    shift it to the left from the leftmost character whose rule is broken
    (slowly swifting characters to the left but the rightmost correct position)
    */

    // TODO: maybe ineffective clone
    let mut reordered = manual.clone();

    for _ in 0..manual.len() // boundary condition, worst case is moving each element once
    {
        match get_first_broken_rule_idx(&reordered, following)
        {
            None => { return reordered; }
            Some(idx) => 
            {
                // guaranteed that idx is safe by called function
                // CLONE must be here -> we can't hold borrowed internals of a object 
                //      (e.g. iterator) and change the object (because after changing 
                //      the object, the reference might become invalid)
                let page = reordered.get(idx).unwrap().clone();

                // since page has broken some rules, its guaranteed that it has some rules => safe
                let rules = following.get(&page).unwrap();

                // we know that some of the elements must be in the rules => safe
                let first_follower_idx = reordered.iter()
                    .position(|i| rules.contains(i))
                    .unwrap();

                // delete old position of faulty page
                reordered.remove(idx);
                // insert it to the new position
                reordered.insert(first_follower_idx, page);
            }
        }
    }

    panic!("Probably iterating for too long over manual: {manual:?}");
}

fn get_first_broken_rule_idx(manual: &Vec<i32>, following: &HashMap<i32, Vec<i32>>) -> Option<usize>
{
    // tracks encountered pages for each iteration (iterating over pages in manual)
    let mut encountered: HashSet<&i32> = HashSet::new();

    for idx in 0..manual.len()
    {
        let page = manual.get(idx).unwrap();

        match following.get(page)
        {
            // no rules (following pages) for this page
            None => (),
            // make sure that no following pages (from the rules) were encountered
            Some(following_pages) => 
            {
                // compute intersection between following pages (according to rules) and encountered pages
                let following_pages_set = HashSet::from_iter(following_pages.iter());
                let intersect: Vec<_> = encountered.intersection(&following_pages_set).collect();

                // intersection should be empty (no following page should have been encountered before)
                if intersect.len() != 0
                {
                    return Some(idx);
                }
            }
        }

        encountered.insert(page);
    }

    // no rule broken
    return None
}

fn is_manual_valid(manual: &Vec<i32>, following: &HashMap<i32, Vec<i32>>) -> bool
{
    match get_first_broken_rule_idx(manual, following)
    {
        None => true,
        Some(_) => false,
    }
}

fn extract_middle_page(manual: &Vec<i32>) -> i32
{
    let idx: usize = manual.len() / 2;
    return match manual.get(idx)
    {
        Some(i) => *i,
        None => panic!("Index {idx} not found in array {manual:?}"),
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let input = get_input(r"D:\src\Advent2024\inputs\05.txt")?;
    // println!("{input:?}");

    let data = transform_input(&input);
    // println!("{data:?}");

    let valid_cnt = count_valid_manuals(&data);
    println!("Result: {valid_cnt}");

    let bonus_cnt = count_invalid_manuals(&data);
    println!("Bonus: {bonus_cnt}");

    // for each page, store pages that can't follow it 
    // everytime when page is added, check that it was not encountered before
    return Ok(());
}


#[cfg(test)]
mod tests 
{
    use super::*;

    // TODO: for the love of god, find a way to parametrize tests
    #[test]
    fn valid_no_prohibited() 
    {
        let result = is_manual_valid(&vec![1, 2, 3], &HashMap::new());
        assert_eq!(result, true);
    }

    #[test]
    fn valid_prohibited() 
    {
        // rule 1|2 -> if 2 is encountered, 1 must have already been encountered
        let preceding = HashMap::from([
            (1, vec![2, 3]),
            (2, vec![3]),
            ]);
        let result = is_manual_valid(&vec![1, 2, 3], &preceding);
        assert_eq!(result, true);
    }

    #[test]
    fn invalid_1() 
    {
        // rule 2|1 -> if 1 is encountered, 2 must have already been encountered
        let preceding = HashMap::from([
            (2, vec![1]),
            ]);
        let result = is_manual_valid(&vec![1, 2, 3], &preceding);
        assert_eq!(result, false);
    }

    #[test]
    fn reorder1() 
    {
        let preceding = HashMap::from([
            (4, vec![2, 3]),
            ]);
        let result = reorder_manual(&vec![1, 2, 3, 4, 5], &preceding);
        assert_eq!(result, vec![1,4,2,3,5]);
    }

    #[test]
    fn reorder2() 
    {
        let preceding = HashMap::from([
            (4, vec![2, 3]),
            (5, vec![3]),
            ]);
        let result = reorder_manual(&vec![1, 2, 3, 4, 5], &preceding);
        assert_eq!(result, vec![1,4,2,5,3]);
    }

    #[test]
    fn reorder3() 
    {
        let preceding = HashMap::from([
            (2, vec![1]),
            (3, vec![2]),
            (4, vec![3]),
            (5, vec![4]),
            ]);
        let result = reorder_manual(&vec![1, 2, 3, 4, 5], &preceding);
        assert_eq!(result, vec![5,4,3,2,1]);
    }

    #[test]
    fn reorder4() 
    {
        let preceding = HashMap::from([
            (3, vec![2]),
            (5, vec![1]),
            ]);
        let result = reorder_manual(&vec![1, 2, 3, 4, 5], &preceding);
        assert_eq!(result, vec![3,2,5,1,4]);
    }

    #[test]
    fn reorder5() 
    {
        let preceding = HashMap::from([
            (3, vec![1]),
            (5, vec![2]),
            ]);
        let result = reorder_manual(&vec![1, 2, 3, 4, 5], &preceding);
        assert_eq!(result, vec![3,1,5,2,4]);
    }


    #[test]
    fn first_broken_valid() 
    {
        let result = get_first_broken_rule_idx(&vec![1, 2, 3], &HashMap::new());
        assert_eq!(result, None);
    }

    #[test]
    fn first_broken_1()
    {
        let preceding = HashMap::from([
            (4, vec![2, 3]),
            ]);
        let result = get_first_broken_rule_idx(&vec![1, 2, 3, 4, 5], &preceding);
        assert_eq!(result, Some(3));
    }
}