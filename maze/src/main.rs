use std::collections::{VecDeque, HashSet, HashMap};
use std::fs::File;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

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
    position: Position,
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
                if bit==1{
                    if index ==0{
                        if(col !=0){
                            let new_position = Position::new(row,col-1);
                            cell.add_position(new_position);
                        }
                    }else if index==1{
                        if(col != 8){
                            let new_position = Position::new(row,col+1);
                            cell.add_position(new_position);
                        }
                    }else if index==2{
                        if(row != -0){
                            let new_position = Position::new(row-1,col);
                            cell.add_position(new_position);
                        }
                    }else{
                        if(row !=5){
                            let new_position = Position::new(row+1,col);
                        cell.add_position(new_position);
                        }
                    }
                    cell.doors.push(doors.chars().nth(index).unwrap() == '1')
                    
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

    fn print(&self){
        println!("({}, {})", self.row + 1, self.col + 1);
    }
}

impl Cell {
    fn new(position:Position) -> Self{
        Cell { position: position, key: false, next_positions: Vec::new(), doors: Vec::new(), exit: false }
    }

    fn add_position(&mut self,elem: Position){
        self.next_positions.push(elem);
    }

    fn get_positions_size(&mut self) ->usize{
        self.next_positions.len()
    }
}

impl Player {
    fn new(row: i8, col: i8) -> Self {
        Player {
            position: Position { row, col },
            key: false
        }
    }

    fn find_exit(&mut self, maze: &Maze) -> (Option<Position>, Vec<Position>) {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(self.position);
        visited.insert(self.position);
        let mut path = vec![self.position.clone()];
        
        while let Some(position) = queue.pop_front() {
            let cell = &maze.positions[position.row as usize][position.col as usize];
            if cell.exit {
                return (Some(position), path);
            }
            if cell.key {
                self.key = true;
            }
            for (i, next_position) in cell.next_positions.iter().enumerate() {
                if !visited.contains(next_position) {
                    if !cell.doors[i] || self.key {
                        visited.insert(next_position.clone());
                        path.push(next_position.clone());
                        queue.push_back(next_position.clone());
                    }
                }
            }
        }
        (None, path)
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

fn filter_path(path: &Vec<Position>) -> Vec<Position> {
    let mut new_path = vec![path[0].clone()];

    for i in 0..path.len() - 1 {
        let current = &path[i];
        let next = &path[i + 1];
        if is_neighbor(current, next) {
            new_path.push(next.clone());
        }
    }

    new_path
}

fn is_neighbor(current: &Position, next: &Position) -> bool {
    (current.row == next.row && (current.col == next.col - 1 || current.col == next.col + 1)) ||
    (current.col == next.col && (current.row == next.row - 1 || current.row == next.row + 1))
}

fn main() {
    let mut maze= Maze::new();
    maze.fill_maze();
    println!("{:#?}",maze);
    let mut player =Player::new(0,0);
    let resp=player.find_exit(&maze);
    let vec = resp.1;
    let final_position = resp.0.unwrap();
    println!("{:#?}",final_position);
    // let filtered = filter_path(&vec);
    // for position in vec{
    //     position.print();
    // }
    // print_movement(&shortest);
    print_movement(&vec);
}
