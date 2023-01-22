use std::collections::{VecDeque, HashSet, HashMap};
use std::fs::File;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::sync::mpsc::channel;
use std::thread;

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


#[derive(Debug,Clone)]
struct Player {
    key: bool
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

    fn fill_maze(&mut self){
        let file = File::open("./src/data/positions.txt").expect("Wrong path to a file!");
        let reader = BufReader::new(file);
    
        let mut row:i8 =0;
        let mut col:i8 =0;
        self.add_empty_row();
        for line in reader.lines() {
            if col==9{
                col=0;
                row+=1;
                self.add_empty_row();
            }
            let position= Position::new(row,col);
            let mut cell = Cell::new(position);
            let line = line.unwrap();
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

impl Player {
    fn new() -> Self {
        Player {
            key: false
        }
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

fn find_exit(maze: &Maze) -> (Option<Position>, Vec<Position>) {
    let mut visited: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
    let mut path: Vec<Position> = vec![];

    fn backtrack(current: &Position, maze: &Maze, visited: &mut Vec<Vec<bool>>, path: &mut Vec<Position>,key: &mut bool) -> Option<Position> {
        println!("{:#?} {}",current,*key);
        if visited[current.row as usize][current.col as usize]{
            return None;
        }
        visited[current.row as usize][current.col as usize] = true;
        path.push(*current);

        let current_cell = &maze.positions[current.row as usize][current.col as usize];
        if current_cell.key{
            *key = true;
        }
        if current_cell.exit {
            return Some(*current);
        }

        for (next_position, &door) in current_cell.next_positions.iter().zip(current_cell.doors.iter()) {
            if door && !*key {
                continue;
            } else if door && *key{
                if let Some(result) = backtrack(next_position, maze, visited, path,&mut false) {
                    return Some(result);
                }
            }else if !door{
                if let Some(result) = backtrack(next_position, maze, visited, path,key) {
                    return Some(result);
                }
            }
        }
        if !current_cell.key{
            path.pop();
        }
        None
    }


    if let Some(result) = backtrack(&Position { row: 0 as i8, col: 0 as i8 }, maze, &mut visited, &mut path,&mut false) {
        return (Some(result), path);
    }

    (None, path)
}


fn main() {
    let mut maze= Maze::new();
    maze.fill_maze();
    println!("{:#?}",maze);
    let exit =find_exit(&maze);
    print_movement(&exit.1);
    println!("{:#?}",exit.0);
    // let filtered = filter_path(&vec);
    // for position in vec{
    //     position.print();
    // }
    // print_movement(&shortest);
    // print_movement(&vec);
}
