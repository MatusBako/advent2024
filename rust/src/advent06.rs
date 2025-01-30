use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::vec;

use std::cell::OnceCell;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction 
{
    Up,
    Right,
    Down,
    Left    
}

impl Direction 
{
    fn rotate(&mut self)
    // clockwise roration
    {
        *self = match self
        {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down=> Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl From<&Direction> for char
{
    fn from(value: &Direction) -> Self
    {
        match value
        {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

// TODO: try implementing Eq and Hash myself
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl From<&Direction> for Position 
{
    fn from(value: &Direction) -> Self 
    {
        match value
        {
            Direction::Up => Position(-1, 0),
            Direction::Right => Position(0, 1),
            Direction::Down=> Position(1, 0),
            Direction::Left => Position(0, -1),
        }
    }
}

impl fmt::Display for Position
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "Position({},{})", self.0, self.1)    
    }
}

// TODO: reduce copying of the same implementation?
impl std::ops::Add<&Position> for &Position {
    type Output = Position;

    fn add(self, _rhs: &Position) -> Position {
        return Position(self.0 + _rhs.0, self.1 + _rhs.1);
    }
}


impl std::ops::Add<&Position> for Position {
    type Output = Position;

    fn add(self, _rhs: &Position) -> Position {
        return Position(self.0 + _rhs.0, self.1 + _rhs.1);
    }
}

impl std::ops::Add<Position> for &Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        return Position(self.0 + _rhs.0, self.1 + _rhs.1);
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, _rhs: Position) -> Position {
        return Position(self.0 + _rhs.0, self.1 + _rhs.1);
    }
}

// TODO: separate guard and maze?
#[derive(Clone, Debug)]
struct Guard
{
    position: Position,
    _position_in_front: OnceCell<Position>,
    direction: Direction,
}

impl Guard
{
    fn new(position: Position, direction: Direction) -> Self
    {
        Guard { position: position, direction: direction, _position_in_front: OnceCell::new() }
    }

    fn position_in_front(&self) -> &Position
    {
        self._position_in_front.get_or_init(|| { self.position + Position::from(&self.direction) });
        self._position_in_front.get().expect("Position should have been initialized.")
    }

    fn rotate(&mut self) -> ()
    {
        self.direction.rotate();
        self._position_in_front = OnceCell::new();
    }

    fn step(&mut self) -> ()
    {
        // TODO: how to do this without move redefining _move_by method or cloning?
        self._move_by(&(Position::from(&self.direction)))
    }

    fn _move_by(&mut self, difference: &Position) -> ()
    {
        self.position = self.position + difference;
        self._position_in_front = OnceCell::new();
    }


}

#[derive(Clone, Debug)]
struct Maze
{
    obstacles: HashSet<Position>,
    size: (i32, i32)
}

#[derive(Debug)]
struct CustomError
{
    message: String
}

impl fmt::Display for CustomError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.message)
    }
}


impl Maze
{    
    fn is_obstacle(&self, position: &Position) -> bool
    {
        self.obstacles.contains(position)
    }

    fn is_position_inside(&self, position: &Position) -> bool
    {
        position.0 >= 0
        && position.1 >= 0
        && position.0 < self.size.0
        && position.1 < self.size.1
    }
}

// for Display
fn safe_write(vec: &mut Vec<u8>, idx: usize, char: u8) -> ()
{
    if idx < vec.len()
    {
        vec[idx] = char;
    }
    else
    {
        // TODO: warning
    }
}

fn print_maze(maze: &Maze, guard: &Guard, visited: Vec<(Position, Direction)>) -> ()
{
    // +2 for \r\n
    let vector_size = (maze.size.0 * (maze.size.1 + 2)) as usize;
    let mut out: Vec<u8> = vec![0; vector_size];
    out.fill(b'.');

    let line_width =  maze.size.1 + 2;

    // newlines
    for row_idx in 0..maze.size.0
    {
        safe_write(&mut out, (row_idx * line_width + maze.size.1) as usize, b'\r');
        safe_write(&mut out, (row_idx * line_width + maze.size.1 + 1) as usize, b'\n');
    }

    let guard_char = char::from(&guard.direction);

    for (position, direction) in visited
    {
        safe_write(&mut out, (position.0 * line_width + position.1) as usize, char::from(&direction) as u8);
    } 

    // guard
    safe_write(&mut out, (guard.position.0 * line_width + guard.position.1) as usize, guard_char as u8);

    // obstacles
    for obstacle in &maze.obstacles
    {
        safe_write(&mut out, (obstacle.0 * line_width + obstacle.1) as usize, b'#');
    }

    match String::from_utf8(out)
    {
        Ok(string) => { println!("{}", string); },
        // TODO: probably not the way to propagate error
        Err(err) => { 
            // TODO: propagate error
            //Err(fmt::Error)
         },
    }

}

fn get_input(file_path: &str) -> Result<(Maze, Guard), Box<dyn std::error::Error>>
{
    if !fs::exists(file_path).is_ok_and(|x| x)
    {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound, 
            "Path \"{file_path}\" does not exist!")));
    }

    let file = fs::File::open(file_path)?;
    let mut reader = io::BufReader::new(file);

    let mut obstacles: Vec<Position> = vec![];
    let mut guard_position: Position = Position(0 , 0);

    let mut buf: [u8; 1] = [0; 1]; //Vec<u8> =  vec![1];
 
    let mut row_idx = 0;
    let mut col_idx = 0;

    let mut row_cnt = 0;
    let mut col_cnt = 0;

    let (mut width, mut height) = (-1, -1);

    loop
    {
        match reader.read_exact(&mut buf)
        {
            Ok(_) => (),
            // EOF 
            Err(_) =>
            {
                row_cnt = row_idx;

                // in case last line was not empty (but a valid part of maze), increment row count
                if col_idx != 0
                {
                    row_cnt += 1;
                }
                break;
            }
        }
        
        let c = std::str::from_utf8(&buf)?;

        match c
        {
            // windows compatibility
            "\r" => (),
            "\n" => 
            {
                // store number of columns on first line
                if col_cnt == 0
                {
                    col_cnt = col_idx;
                }

                row_idx += 1;
                col_idx = 0;
            },
            // empty space
            "." => { col_idx += 1 },
            // obstacle
            "#" =>
            {
                obstacles.push(Position(row_idx, col_idx));
                col_idx += 1;
            },
            // guard
            "^" => 
            { 
                guard_position = Position(row_idx, col_idx);
                col_idx += 1;
            },
            _ => { return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid character \"{c}\""))); }
            
        }
    }

    // no need to increment row_idx, it's incremented by the last empty line
    return Ok((
        Maze 
        { 
            size: (row_cnt, col_cnt),
            obstacles: HashSet::from_iter(obstacles)
        }, 
        Guard::new(guard_position, Direction::Up)
    ));
}

#[derive(PartialEq)]
enum TraversalEnd
{
    Outside,
    Cycle,
}

fn count_visited_positions(maze: Maze, mut guard: Guard) -> (TraversalEnd, HashSet<Position>)
{
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    let mut end: Option<TraversalEnd> = None;

    visited.insert((guard.position.clone(), guard.direction.clone()));

    loop
    {
        //println!("Guard: {:?}, direction: {:?}", maze.guard, maze.guard_direction);
        
        // obstacle or wall in front of me
        if maze.is_obstacle(guard.position_in_front())
        {
            guard.rotate();
            
            //println!("{maze}\r\n");
            continue;
        }
        // guard walked through the outside wall
        else if !maze.is_position_inside(guard.position_in_front()) 
        {   
            end = Some(TraversalEnd::Outside);
            break;
        }
        
        // space in front of me was visited while facing the same direction, prevent cycle
        // TODO: without clone?
        if visited.contains(&(*guard.position_in_front(), guard.direction.clone()))
        {
            end = Some(TraversalEnd::Cycle);
            // TODO: debugging print after every cycle
            //print_maze(&maze, &guard, visited.clone().into_iter().collect());
            //let stdin = io::stdin();
            //let mut iterator = stdin.lock().lines();
            //let _ = iterator.next().unwrap();
            break;
        }

        // valid tile in front of guard
        guard.step();
        //println!("{maze}\r\n");

        visited.insert((guard.position.clone(), guard.direction.clone()));
    }

    //println!("Visited all: {}", visited.len());

    // return set of visited positions
    let visited_set: HashSet<Position> =  visited
    .iter()
    .map(|(pos, _dir)| pos.clone())
    .collect::<HashSet<Position>>();

    return (end.expect("Traversal should have ended by exiting maze or cycle!"), visited_set);
}

fn count_obstacle_locations(maze: &Maze, mut guard: Guard) -> usize
{
    let mut cycle_creating_positions: HashSet<Position> = HashSet::new();

    loop
    {        
        if !maze.is_position_inside(guard.position_in_front())
        {
            // guard walked through the outside wall
            break;
        }

        if maze.is_obstacle(guard.position_in_front())
        {
            guard.rotate();
            continue;
        }

        // add obstacle, run traversal, observe result, remove, step
        let mut new_maze = maze.clone();
        new_maze.obstacles.insert(*guard.position_in_front());
        let (ending, _) = count_visited_positions(new_maze, guard.clone());

        if ending == TraversalEnd::Cycle
        {
            cycle_creating_positions.insert(*guard.position_in_front());
        }

        guard.step();
    }

    cycle_creating_positions.len()

}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let io_before = std::time::Instant::now();

    // TODO: separate guard from maze
    let (maze, guard) = get_input(r"D:\src\Advent2024\inputs\06.txt").expect("Input loading failed.");
    let io_duration = io_before.elapsed();
    
    let part1_before = std::time::Instant::now();
    let (_traversal, visited_positions) = count_visited_positions(maze.clone(), guard.clone());
    let part1_duration = part1_before.elapsed();

    let visited_count = visited_positions.len();

    let part2_before = std::time::Instant::now();
    let obstacle_locations = count_obstacle_locations(&maze, guard);
    let part2_duration = part2_before.elapsed();

    println!("Visited positions: {visited_count}");
    println!("I/O duration: {io_duration:#?}");
    println!("Part 1 duration: {part1_duration:#?}");

    println!("Positions creating cycle: {obstacle_locations}");
    println!("Part 2 duration: {part2_duration:#?}");

    return Ok(());
}