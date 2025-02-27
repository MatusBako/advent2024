use std::{fmt::Debug, io::{self, BufRead}, time::Duration};

// clone added due to parallel computation
#[derive(Clone)]
struct Line
{
    product: i64,
    numbers: Vec<i32>,
}

impl std::fmt::Debug for Line
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> 
    {
        // Refactor notes: ineffective, write strings immediately instead of creating and writing
        /*
        let numbers = self.numbers
        .iter()
        .map(i32::to_string)
        .fold(String::new(),
            |mut a, b| 
        {
            // note: this is faster than just concatenating string, because by doing that
            // you are creating a new string each time
            a.reserve(b.len() + 1);

            if a.len() > 0
            {
                a.push_str(" ");
            }
            a.push_str(b.as_str());
            return a;
        });

        write!(f, "[{}: {}]", self.product, numbers)?;
        */
        
        write!(f, "[{}: ", self.product)?;

        for (idx, number) in self.numbers.iter().enumerate()
        {
            let number = i32::to_string(number);

            if idx == 0
            {
                write!(f, "{}", number)?;
            }
            else
            {
                write!(f, ", {}", number)?;
            }
        }
    
        write!(f, "]")?;

        return Ok(());
    }
}

enum Operator {
    Add,
    Mul,
    Cat,
}

impl Operator{
    fn apply(&self, a: i64, b: i64) -> i64
    {
        match self
        {
            Operator::Add => a + b,
            Operator::Mul => a * b,
            Operator::Cat => 
            {
                a * (10 as i64).pow(b.ilog10() + 1) + b
            }
        }
    }
}

#[derive(Clone, Copy)]
struct SearchState
{
    // Refactor note: keeping operators here is redundant if I keep number of operators applied
    // operators: Vec<Operator>,
    
    numbers_consumed: usize,

    // result with operators applied so far
    partial_result: i64,
}

impl SearchState {
    fn new(initial_result: i64) -> SearchState
    {
        // we expect that there is at least one number
        return SearchState { numbers_consumed: 1, partial_result: initial_result } // instead of as i64
    }

    fn add (&mut self, operator: &Operator, number: i64)
    {
        /*
        // Refactor note: why did I push the operator first and then retrieved it from the vector?
        self.operators.push(operator);

        // apply given operator on the result computed so far and next number in line
        // first unwrap safe, the vector is definitely not empty
        // TODO: handle second unwrap
        self.partial_result = self.operators.last().unwrap().apply(self.partial_result, *line.numbers.get(self.operators.len()).unwrap() as i64);
        */
        self.numbers_consumed += 1;
        self.partial_result = operator.apply(self.partial_result, number) // instead of as i64
    }
}

#[derive(Debug)]
struct Input
{
    lines: Vec<Line>,
}

fn get_input(file_path: &str) -> Result<Input, Box<dyn std::error::Error>>
{
    if !std::path::Path::new(file_path).exists()
    {
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, format!("Input file not found: \"{file_path}\""))));
    }

    let file = std::fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<Line> = Vec::new();
    
    for line_result in reader.lines()
    {
        if line_result.is_err()
        {
            // maybe not the best error handling
            return Err(Box::new(line_result.unwrap_err()))
        }

        let line = line_result?;

        // last empty line
        if line.is_empty() || line.as_str().chars().all(|c| c.is_whitespace())
        {
            break;
        }

        let mut split = line.as_str().split(":");

        let product_opt = split.next();
        let numbers_opt = split.next();
        let end_opt = split.next();

        if product_opt.is_none() || numbers_opt.is_none() || end_opt.is_some()
        {
            println!("{line}");
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
                format!("Encountered unexpected behaviour while splitting by \":\" on following line: {line}"))))
        }

        if product_opt.is_none()
        {
            println!("{line}");
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
                format!("Couldn't convert result number to int on line: {line}"))));
        }

        // unwrap is safe
        let product = product_opt.unwrap().parse::<i64>()?;
        let numbers = numbers_opt.unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<_>, _>>();

        if numbers.is_err()
        {
            println!("{line}");
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, 
                format!("Couldn't convert one of numbers to int on line: {line}"))));
        }
        
        lines.push(Line { product: product, numbers: numbers.unwrap()  });
    }

    return Ok(Input { lines: lines });
}

fn compute_total_calibration_result(input: &Input) -> i64
{
    let mut result = 0;

    for line in &input.lines
    {
        result += compute_single_calibration_result(line, vec![Operator::Add, Operator::Mul]);
    }

    return result;
}


fn compute_total_ternary_calibration_result(input: &Input) -> i64
{
    let mut result = 0;

    for line in &input.lines
    {
        result += compute_single_calibration_result(line, vec![Operator::Add, Operator::Mul, Operator::Cat]);
    }

    return result;
}

fn compute_total_parallel_ternary_calibration_result(input: &Input) -> i64
{
    use rayon::prelude::ParallelIterator;
    use rayon::iter::IntoParallelIterator;
    // TODO: without cloning?

    let result = input.lines.clone().into_par_iter()
        .map(|line: Line|
            compute_single_calibration_result(
                &line.clone(),
                vec![Operator::Add, Operator::Mul, Operator::Cat])
            )
        .reduce(|| 0, |x, y| x + y);

    return result;
}

fn _init_search_queue(line: &Line) -> Vec<SearchState>
{
    let mut search_queue: Vec<SearchState> = Vec::new();

    // TODO: why whould I push initial state 3 times if no operator was yet used?
    /*
    for operator in operators_used.iter()
    {
        // numbers is expected to be non-empty
        search_queue.push(SearchState::new(line.numbers[0].into()));
    }
    */
    search_queue.push(SearchState::new(line.numbers[0].into()));

    return search_queue
}

fn compute_single_calibration_result(line: &Line, operators_used: Vec<Operator>) -> i64
{
    let mut search_queue = _init_search_queue(line);
    let mut state: SearchState;

    loop 
    {
        match search_queue.pop()
        {
            Some(elem) => { state = elem; }
            None => { break; }
        };

        
        // if partial result > expected, continue
        if state.partial_result > line.product
        {
            continue;
        }
        
        // if no more operators can be added
        if state.numbers_consumed == line.numbers.len()
        {
            // result equal to expected, success
            if line.product == state.partial_result
            {
                return line.product
            }

            // else, continue search
            continue;
        }        

        // alternating between Add and Mul yields no time saved
        // let (op1, op2) = match (searched.operators.len() as i32) % 2
        // {
        //     0 => (Operator::Add, Operator::Mul),
        //     1 => (Operator::Mul, Operator::Add),
        //     _ => panic!("Makes no sense for modulo"),
        // };

        for operator in operators_used.iter()
        {
            let mut new_state = state; // thanks to Copy
            new_state.add(operator, line.numbers[state.numbers_consumed].into());
            search_queue.push(new_state);
        }
    }
    
    return 0;
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let io_before = std::time::Instant::now();
    let input = get_input(r"D:\src\Advent2024\inputs\07.txt").expect("Input loading failed.");
    let io_duration = io_before.elapsed();

    let part1_before = std::time::Instant::now();
    let calibration_result = compute_total_calibration_result(&input);
    let part1_duration = part1_before.elapsed();

    let part2_before = std::time::Instant::now();
    let bonus_result = compute_total_ternary_calibration_result(&input);
    let part2_duration = part2_before.elapsed();

    println!("I/O duration: {io_duration:#?}");
    println!("Part 1 result: {calibration_result:?}");
    println!("Part 1 duration: {part1_duration:#?}");
    println!("");
    println!("Part 2 result: {bonus_result:?}");
    println!("Part 2 duration: {part2_duration:#?}");
    println!("");

    let mut parallel_time: Duration;
    let mut parallel_before;

    for thread_cnt in 2..17
    {
        parallel_before = std::time::Instant::now();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(thread_cnt)
            .build()?;

        let _ = pool.install(|| compute_total_parallel_ternary_calibration_result(&input));
        parallel_time = parallel_before.elapsed();

        // println!("Parallel result: {parallel_result:#?}");
        println!("Parallel duration for {} threads: {:?}, speedup: {:.1}x, efficiency: {:.2}",
            thread_cnt, parallel_time, part2_duration.div_duration_f32(parallel_time),
            part2_duration.div_duration_f32(parallel_time * (thread_cnt as u32)));
    }

    return Ok(());
}