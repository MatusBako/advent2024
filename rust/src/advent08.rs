use std::{collections::{HashMap, HashSet}, fmt::write, io::BufRead};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate
{
    x: u32,
    y: u32
}

impl std::fmt::Debug for Coordinate
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({}, {})", self.y, self.x)?;
        Ok(())
    }
}

// takes references as parameters but returns value
impl std::ops::Add for &Coordinate
{
    type Output = Coordinate; // not Self this time

    fn add(self, rhs: &Coordinate) -> Coordinate // or Self::Output
    {
        Coordinate{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl std::ops::Sub for &Coordinate
{
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Coordinate
    {
        // todo ensure that vertical difference is always positive
        let (x, y) = if self.y > rhs.y 
            { (self.x - rhs.x, self.y - rhs.y) }
            else { (rhs.x - self.x, rhs.y - self.y) };
        Coordinate{x: x, y: y}
    }
}

#[derive(Debug)]
struct Input
{
    map_size: Coordinate,
    // antena name corresponds to set of locations
    antena_locations: HashMap<char, HashSet<Coordinate>>,
}

fn get_input(filepath: &str) -> Result<Input, std::io::Error>
{
    if !std::path::Path::new(filepath).exists()
    {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, 
            "Input path does not exist!"));
    }

    let file = std::fs::File::open(filepath)?;
    let reader = std::io::BufReader::new(file);

    let (mut line_length, mut line_count) = (0, 0);
    let mut antena_locations: HashMap<char, HashSet<Coordinate>> = HashMap::new();


    for (line_idx, line) in reader.lines().enumerate()
    {
        match line
        {
            Ok(line) => 
            {
                if line_idx == 0
                {
                    // OS independent
                    line_length = line.chars().filter(|c| !c.is_whitespace()).count();
                }

                if line.chars().all(|c| c.is_whitespace())
                {
                    break;
                }

                for (idx, antena) in  line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '.')
                {
                    let coords = Coordinate{ y: line_idx as u32, x:idx as u32};

                    antena_locations.entry(antena)
                        .and_modify(|e: &mut HashSet<Coordinate>| { e.insert(coords); })
                        .or_insert(HashSet::from_iter(vec![coords]));
                }

                line_count = line_idx + 1;
            },
            Err(err) => 
            { 
                return Err(err.into());
            }
        }
    }

    let map_size = Coordinate{x: line_length as u32, y: line_count as u32 };

    return Ok(Input { map_size: map_size, antena_locations: antena_locations });
}

fn find_antinodes(antena1: &Coordinate, antena2: &Coordinate, map_size: &Coordinate, output: &mut Vec<Coordinate>)
{
    let diff = antena1 - antena2;

    let (a_high, a_low) = if antena1.y > antena2.y
    {
        (antena1 + &diff, antena2 - &diff)
    }
    else
    {
        (antena2 + &diff, antena1 - &diff)
    };

    // TODO: antinode between

    if diff.x % 3 == 0 && diff.y % 3 == 0
    {
        let diff_between = Coordinate{x: diff.x / 3, y: diff.y / 3};

        output.push(&a_low + &diff_between);
        output.push(&a_high - &diff_between);
    }

    for antinode in [a_high, a_low]
    {
        if is_antinode_valid(&antinode, map_size)
        {
            output.push(antinode);
        }
    }
}

fn is_antinode_valid(antinode: &Coordinate, map_size: &Coordinate) -> bool
{
    antinode.x >= 0
    && antinode.y >= 0
    && antinode.x < map_size.x
    && antinode.y < map_size.y
}

// fn combinations<I,T>(it1: &I, it2: &I) -> impl Iterator<Item=(T, T)>
// where
//     I: Iterator,
// {
// }

// impl Iterator for (HashSet<Coordinate>, HashSet<Coordinate>)
// {
//     type Item = Coordinate;

//     fn next(&mut self) -> Option<Self::Item> {
        
//     }
// }

fn count_antinode_locations(input: &Input) -> usize
{
    let mut antinode_locations: HashMap<char, HashSet<Coordinate>> = HashMap::new();

    for (freq, locations) in &input.antena_locations
    {
        for loc1 in locations
        {
            for loc2 in locations
            {
                if loc1 == loc2 { continue; }
                
                // TODO: optimize so that vector is created only once
                // however, there are problems with &mut and moving if moved out of loop
                let mut tmp_antinodes: Vec<Coordinate> = Vec::with_capacity(2);

                find_antinodes(loc1, loc2, &input.map_size, &mut tmp_antinodes);

                match antinode_locations.get_mut(freq)
                {
                    Some(antennas) => {
                        antennas.extend(tmp_antinodes); 
                    },
                    None => { 
                        antinode_locations.insert(*freq, HashSet::from_iter(tmp_antinodes)); 
                    }
                }

                //tmp_antinodes.clear();
            }
        }
    }
    
    // TODO: ineffective, creates new hash set every time
    // use into iter and insert one by one

    let result = antinode_locations
        .iter()
        .map(|(c, set)| set)
        .fold(HashSet::new(), |result: HashSet<Coordinate>, elem|
            result.union(elem).copied().collect() ).len();

    // .iter_mut()
    // .map(|(c, set)| set)
    // .reduce(|set1, set2|
    //     set1.union(set2))
    // .and_then(|(c, set)| Some(set.len()))
    // .or(Some(0));

    return result;
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let io_before = std::time::Instant::now();
    let input = get_input(r"D:\src\Advent2024\inputs\08.txt")
        .expect("Input loading failed.");
    let io_duration = io_before.elapsed();

    let part1_before = std::time::Instant::now();
    let part1_result = count_antinode_locations(&input);
    let part1_duration = part1_before.elapsed();

    //println!("Input: {input:#?}");
    println!("Part 1 result: {part1_result}");

    return Ok(());
}