use core::panic;
use std::collections::{VecDeque, HashSet, HashMap};
use std::fs::File;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::thread;
use std::sync::mpsc::{self, Sender};
use std::io::Write;
use std::time::Instant;

#[derive(Debug,Clone)]
struct Cell{
    position: Position,
    key: bool,
    next_positions: Vec<Position>,
    doors: Vec<bool>,
    exit: bool  
}

#[derive(Debug,Clone,Copy)]
struct Position{
    row: i8,
    col: i8,
}

impl Eq for Position {}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

#[derive(Debug,Clone)]
struct Maze{
    positions: Vec<Vec<Cell>>
}


impl Maze {
    fn new() ->Self{
        Maze { positions: Vec::new() }
    }

    fn add_empty_row(&mut self) {
        self.positions.push(vec![]);
    }

    fn add_cell(&mut self, new_cell: Cell) {
        if let Some(last_row) = self.positions.last_mut() {
            last_row.push(new_cell);
        }
    }

    fn fill_maze(&mut self) -> Result<(), std::io::Error>{
        let file = File::open("./src/data/positions.txt")?;
        let reader = BufReader::new(file);
    
        let mut row:i8 =0;
        let mut col:i8 =0;
        self.add_empty_row();
        for line in reader.lines(){
            let line= line?;
            if col==9{
                col=0;
                row+=1;
                self.add_empty_row();
            }
            let position= Position::new(row,col);
            let mut cell = Cell::new(position);
            let mut splited = line.split(" ");
            let positions=  splited.next().unwrap().to_owned();
            let bits = positions.chars().map(|c| c.to_digit(10).unwrap());
    
            let doors=  splited.next().unwrap().to_owned();
            for (index,bit) in bits.enumerate() {
                if bit==1 {
                    if index ==0{
                        if col !=0 {
                            let new_position = Position::new(row,col-1);
                            cell.add_position(new_position);
                            cell.doors.push(doors.chars().nth(index).unwrap() == '1')
                        }
                    }else if index==1{
                        if col != 8 {
                            let new_position = Position::new(row,col+1);
                            cell.add_position(new_position);
                            cell.doors.push(doors.chars().nth(index).unwrap() == '1')
                        }
                    }else if index==2{
                        if row != -0 {
                            let new_position = Position::new(row-1,col);
                            cell.add_position(new_position);
                            cell.doors.push(doors.chars().nth(index).unwrap() == '1')
                        }
                    }else{
                        if row !=5 {
                            let new_position = Position::new(row+1,col);
                        cell.add_position(new_position);
                        cell.doors.push(doors.chars().nth(index).unwrap() == '1')
                        }
                    }
                    
                }
            }
            let thrd_el = splited.next().unwrap().to_owned();
            let first_two_bits = &thrd_el[0..2];
            let last_two_bits = &thrd_el[thrd_el.len()-2..thrd_el.len()];
            cell.key = first_two_bits == "11";
            cell.exit = last_two_bits == "11";
            self.add_cell(cell);
    
            col +=1;
        }
        Ok(())
    }
    
}

impl Position{
    fn new(row: i8,col:i8) -> Self{
        Self { row: row, col: col }
    }
}

impl Cell {
    fn new(position:Position) -> Self{
        Cell { position: position, key: false, next_positions: Vec::new(), doors: Vec::new(), exit: false }
    }

    fn add_position(&mut self,elem: Position){
        self.next_positions.push(elem);
    }
}

fn print_movement(movement: &Vec<Position>) {
    let max_row = 5;
    let max_col = 8;

    for row in 0..=max_row {
        for col in 0..=max_col {
            if movement.contains(&Position{row,col}){
                print!("| ");
            } else {
                print!("* ");
            }
        }
        println!();
    }
}

fn find_exit(maze: &Maze) -> Vec<Vec<Position>> {
    let (tx, rx) = mpsc::channel();
    let maze_clone = maze.clone();
    thread::spawn(move || {
        let mut visited = vec![vec![false; 9]; 9];
        let mut key_positions = HashSet::new();
        let mut keys = 0;
        let mut path = vec![];
        backtrack_parallel(&Position { row: 0 as i8, col: 0 as i8 }, &maze_clone, &mut visited, &mut path, &mut keys, &mut key_positions, tx);
    });
    let mut result: Vec<Vec<Position>> = vec![];
    while let Ok(path) = rx.recv() {
        result.push(path);
    }
    result
}

fn backtrack_parallel(current: &Position, maze: &Maze, visited: &mut Vec<Vec<bool>>, path: &mut Vec<Position>, keys: &mut i32, key_positions: &mut HashSet<Position>, tx: mpsc::Sender<Vec<Position>>) {
    if visited[current.row as usize][current.col as usize] {
        return;
    }
    visited[current.row as usize][current.col as usize] = true;
    path.push(*current);

    let current_cell = &maze.positions[current.row as usize][current.col as usize];
    if current_cell.key && !key_positions.contains(current) {
        *keys += 1;
        key_positions.insert(*current);
    }
    if current_cell.exit {
        tx.send(path.clone());
    }

    for (next_position, &door) in current_cell.next_positions.iter().zip(current_cell.doors.iter()) {
        if door && *keys == 0 {
            continue;
        } else if door && *keys > 0 {
            *keys -= 1;
            let tx_clone=tx.clone();
            backtrack_parallel(next_position, maze, visited, path, keys, key_positions, tx_clone);
            *keys += 1;
        } else if !door {
            let tx_clone=tx.clone();
            backtrack_parallel(next_position, maze, visited, path, keys, key_positions, tx_clone);
        }
    }
    if !current_cell.key {
        path.pop();
    }
}


fn find_exit_sequentially(maze: &Maze) -> Vec<Vec<Position>> {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
    let mut result: Vec<Vec<Position>> = vec![];
    let mut key_positions: HashSet<Position> = HashSet::new();
    
    fn backtrack(current: &Position, maze: &Maze, visited: &mut Vec<Vec<bool>>, path: &mut Vec<Position>, keys: &mut i32, key_positions: &mut HashSet<Position>, result: &mut Vec<Vec<Position>>) {
        if visited[current.row as usize][current.col as usize] {
            return;
        }
        visited[current.row as usize][current.col as usize] = true;
        path.push(*current);
    
        let current_cell = &maze.positions[current.row as usize][current.col as usize];
        if current_cell.key && !key_positions.contains(current) {
            *keys += 1;
            key_positions.insert(*current);
        }
        if current_cell.exit {
            result.push(path.clone());
        }
    
        for (next_position, &door) in current_cell.next_positions.iter().zip(current_cell.doors.iter()) {
            if door && *keys == 0 {
                continue;
            } else if door && *keys > 0 {
                *keys -= 1;
                backtrack(next_position, maze, visited, path, keys, key_positions, result);
                *keys += 1;
            } else if !door {
                backtrack(next_position, maze, visited, path, keys, key_positions, result);
            }
        }
        if !current_cell.key {
            path.pop();
        }
    }
    
   
   let mut path: Vec<Position> = vec![];
   let mut keys = 0;
    backtrack(&Position { row: 0 as i8, col: 0 as i8 }, maze, &mut visited, &mut path, &mut keys, &mut key_positions, &mut result);
   
   result
   }
   


fn main() {
    let mut maze= Maze::new();
    let fill=maze.fill_maze();
    match fill {
        Ok(())=>{
            let mut file = File::create("output.txt").unwrap();
            let output = format!("{:#?}",maze);
            file.write_all(output.as_bytes()).unwrap();
            // println!("{:#?}",maze);
            let start_time_sequentially = Instant::now();
            let exit_sequentially =find_exit_sequentially(&maze);
            let end_time_sequentially = Instant::now();
        
            let duration_sequentially = end_time_sequentially.duration_since(start_time_sequentially);
            println!("{}",duration_sequentially.as_micros());
            let start_time = Instant::now();
            let exit =find_exit(&maze);
            let end_time = Instant::now();
        
            let duration = end_time.duration_since(start_time);
            println!("{}",duration.as_micros());
            for e in exit{
                println!("###############################################");
                print_movement(&e);
            }
        },
        Err(e) => panic!("{}",e),
    }
}
