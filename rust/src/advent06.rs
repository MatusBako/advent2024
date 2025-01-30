use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::io::Read;
use std::vec;


// TODO: try implementing Eq and Hash myself
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(i32, i32);

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
#[derive(Debug)]
struct Guard
{
    position: Position,
    direction: Position,
}

impl Guard
{
    fn rotate_guard(&mut self) -> ()
    {
        match (self.direction.0, self.direction.1)
        {
            (-1, 0) => { self.direction = Position(0, 1)},
            (0, 1) => { self.direction = Position(1, 0)},
            (1, 0) => { self.direction = Position(0 , -1)},
            (0, -1) => { self.direction = Position(-1, 0)},
            _ => panic!("Tried to rotate guard with direction ({})", self.direction)
        }
    }

    fn step(&mut self) -> ()
    {
        // clone required due to mutable reference
        // TODO: how to do this without move redefining _move_by method or cloning?
        self._move_by(&(self.direction.clone()))
    }

    fn _move_by(&mut self, direction: &Position) -> ()
    {
        self.direction = self.direction + direction
    }


}

#[derive(Clone, Debug)]
struct Maze
{
    guard: Position,
    guard_direction: Position,
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
    fn is_obstacle_in_front(&self) -> bool
    {
        self.obstacles.contains(&(self.guard + self.guard_direction))
    }
    
    fn _is_obstacle(&self, position: &Position) -> bool
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

    fn move_guard_in_direction(&mut self) -> Result<(), CustomError>
    {
        // clone required due to mutable reference
        // TODO: how to do this without move redefining method or cloning?
        let direction =  self.guard_direction.clone();
        self.move_guard_by(&direction)
    }

    fn move_guard_by(&mut self, position: &Position) -> Result<(), CustomError>
    {
        let dest: &Position = &(self.guard + position);
        if self.is_position_inside(dest) && !self._is_obstacle(&(self.guard + position))
        {
            self.guard = self.guard + position;
            return Ok(());
        }
        Err(CustomError { message: 
            format!("Tried to move guard {} by {}", self.guard, position)})
    }
    
    fn rotate_guard(&mut self) -> ()
    {
        match (self.guard_direction.0, self.guard_direction.1)
        {
            (-1, 0) => { self.guard_direction = Position(0, 1)},
            (0, 1) => { self.guard_direction = Position(1, 0)},
            (1, 0) => { self.guard_direction = Position(0 , -1)},
            (0, -1) => { self.guard_direction = Position(-1, 0)},
            _ => panic!("Tried to rotate guard with direction ({})", self.guard_direction)
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
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        // +2 for \r\n
        let vector_size = (self.size.0 * (self.size.1 + 2)) as usize;
        let mut out: Vec<u8> = vec![0; vector_size];
        out.fill(b'.');

        let line_width =  self.size.1 + 2;

        // newlines
        for row_idx in 0..self.size.0
        {
            Maze::safe_write(&mut out, (row_idx * line_width + self.size.1) as usize, 
                b'\r');
            Maze::safe_write(&mut out, (row_idx * line_width + self.size.1 + 1) as usize,
                b'\n');
        }

        let guard: char = match self.guard_direction
        {
            Position(-1, 0) => '^',
            Position(0, 1) => '>',
            Position(1, 0) => 'v',
            Position(0, -1) => '<',
            _ => 'X',
        };

        // guard
        Maze::safe_write(&mut out, (self.guard.0 * line_width + self.guard.1) as usize,
        guard as u8);

        // obstacles
        for obstacle in &self.obstacles
        {
            
            Maze::safe_write(&mut out, (obstacle.0 * line_width + obstacle.1) as usize,
                b'#');
        }

        match String::from_utf8(out)
        {
            Ok(string) => write!(f, "{}", string),
            Err(err) => Err(fmt::Error),
        }

    }
}

fn get_input(file_path: &str) -> Result<Maze, Box<dyn std::error::Error>>
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
    let mut guard: Position = Position(0 , 0);

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
                guard = Position(row_idx, col_idx);
                col_idx += 1;
            },
            _ => { return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid character \"{c}\""))); }
            
        }
    }

    // no need to increment row_idx, it's incremented by the last empty line
    return Ok(Maze { guard: guard, obstacles: HashSet::from_iter(obstacles), size: (row_cnt, col_cnt),
        guard_direction: Position(-1, 0) });
}

fn count_visited_positions(mut maze: Maze) -> HashSet<Position>
{
    let mut visited: Vec<(Position, Position)> = vec![(maze.guard.clone(), maze.guard_direction.clone())];

    loop
    {
        //println!("Guard: {:?}, direction: {:?}", maze.guard, maze.guard_direction);
        
        // obstacle or wall in front of me
        if maze.is_obstacle_in_front()
        {
            maze.rotate_guard();
            
            //println!("{maze}\r\n");
            continue;
        }
        // guard walked through the outside wall
        else if !maze.is_position_inside(&(maze.guard + maze.guard_direction)) 
        {
            break
        }
        // space in front of me was visited while facing the same direction, prevent cycle
        if visited.contains(&(maze.guard + maze.guard_direction, maze.guard_direction))
        {
            break;
        }

        // valid tile in front of guard
        maze.move_guard_in_direction();
        //println!("{maze}\r\n");

        visited.push((maze.guard.clone(), maze.guard_direction.clone()));
    }

    println!("Visited all: {}", visited.len());

    // return set of visited positions
    return visited
    .iter()
    .map(|(pos, _dir)| pos.clone())
    .collect::<std::collections::HashSet<Position>>();
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let maze = get_input(r"D:\src\Advent2024\inputs\06.txt").expect("Input loading failed.");
    
    //println!("{maze}\r\n");
    //return Ok(());

    let visited_positions = count_visited_positions(maze);
    let visited_count = visited_positions.len();

    print!("Visited positions: {visited_count}");

    return Ok(());
}