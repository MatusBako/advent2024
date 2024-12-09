use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::zip;

fn get_input() -> Result<Vec<(i32, i32)>, io::Error>
{
    let file_path = r"D:\src\Advent2025\inputs\01.txt";

    if !fs::exists(file_path).is_ok()
    {
        panic!("File not found");
    }

    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Easy
    // let result: Vec<(i32, i32)>;
    // for line in reader.lines()
    // {
    //     result.append(parse_line(line));
    // }

    // Classy
    return Ok(reader.lines().map(parse_line).collect());
}

fn parse_line(line: io::Result<String>) -> (i32, i32)
{
    // Easy option
    // let mut result: Vec<i32> = vec![];

    //     for number in line.unwrap().split_whitespace()
    // {
    //     result.push(number.parse::<i32>().unwrap());
    // }

    // Advanced option
    //let result: Vec<i32> = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let result: Vec<i32> = line.unwrap().split_whitespace().map(|x: &str| match x.parse::<i32>() {
            Ok(v) => v,
            Err(e) => panic!("Error while parsing string {x}: {e}"),
    }).collect();

    if result.len() != 2
    {
        // TODO: how to print it without debug symbols?
        panic!("Incorrect number of integers: {result:?}");
    }

    return (result[0], result[1])
}

fn compute(vector: &Vec<(i32, i32)>) -> i32
{
    // stupid way
    /* vector.sort_by(
        |x: &(i32, i32), y: &(i32, i32)| match x.0 < y.0
    {
        true => Ordering::Less,
        false => Ordering::Greater,
    });
    */

    // better way
    //vector.sort_by(|x: &(i32, i32), y: &(i32, i32)| x.cmp(y));

    // best way
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = (
        vector.clone().iter().map(|x| x.0).collect(),
        vector.clone().iter().map(|x| x.1).collect());
    first.sort();
    second.sort();

    let result: i32 = zip(first, second)
    .map(|x| x.0.abs_diff(x.1) as i32)
    //.map(|x|  (x.0 - x.1).abs())
    //.fold(0, |acc, el| acc + el);
    .sum();

    return result;
}

fn compute_bonus(vector: &Vec<(i32, i32)>) -> i32
{
    let mut counts: HashMap<i32, i32> = HashMap::new();

    // count occurences of numbers in right column
    // also, the ugliest shit I've written so far in this
    // TODO: without declaring throwaway variable but keeping type?
    let _: Vec<Option<i32>> = vector.iter().map(|x: &(i32, i32)| {
        counts.insert(x.1, match counts.get(&x.1)
        {
            Some(i) => i + 1,
            None => 1
        })
    }).collect();

    print!("{counts:?}");

    // TODO: without unwrap? 
    // why are the ampersands here? still didn't learn borrow mechanic?
    return vector.iter().map(|x| &x.0 * counts.get(&x.0).unwrap_or(&0)).sum();
}

fn main()
{
    // TODO: make it return vector only using ?
    let input: Result<Vec<(i32, i32)>, io::Error> = get_input();
    //let input_str = parse_line(Ok("123 321".to_string()));

    // TODO: preco to chce nejaku referenciu/kopiu? asi si musim nastudovat borrow a pod.
    //println!("Processed {n} lines", n=input.as_ref().unwrap().len());

    let (result, bonus_result) = match input
    {
        Ok(v) => 
            (compute(&v), compute_bonus(&v)),
        Err(error) => {
            panic!("Error on I/O: {e:?}", e=error)
        },
    };

    println!("Result: {result}");
    println!("Bonus result: {bonus_result}");
}


// potrebne ak chcem pouzit ? v maine - aby vybublal error
// Box asi preto lebo Error je dynamicky alokovana struktura na heape
// tzn dopredu neviem kolko zaberie pamate
fn _main2() -> Result<(), Box<dyn std::error::Error>>
{
    let input = get_input()?;

    println!("Result: {res}", res=compute(&input));
    println!("Bonus result: {bonus}", bonus=compute_bonus(&input));
    return Ok(());
}

