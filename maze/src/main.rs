use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug,Clone)]
struct Cell{
    position: Position,
    key: bool,
    next_positions: Vec<Position>,
    doors: Vec<bool>,
    exit: bool  
}

#[derive(Debug,Clone)]
struct Position{
    row: i8,
    col: i8
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

    fn get_positions_size(&mut self) ->usize{
        self.next_positions.len()
    }
}

fn fill_maze(maze: &mut Maze){
    let file = File::open("./src/data/positions.txt").expect("Wrong path to a file!");
    let reader = BufReader::new(file);

    let mut row:i8 =1;
    let mut col:i8 =1;
    maze.add_empty_row();
    for line in reader.lines() {
        if row==10{
            row=1;
            col+=1;
            maze.add_empty_row();
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
                    let new_position = Position::new(row-1,col);
                    cell.add_position(new_position);
                }else if index==1{
                    let new_position = Position::new(row+1,col);
                    cell.add_position(new_position);
                }else if index==2{
                    let new_position = Position::new(row,col-1);
                    cell.add_position(new_position);
                }else{
                    let new_position = Position::new(row,col+1);
                    cell.add_position(new_position);
                }
                cell.doors.push(doors.chars().nth(index).unwrap() == '1')
                
            }
        }
        let thrd_el = splited.next().unwrap().to_owned();
        let first_two_bits = &thrd_el[0..2];
        let last_two_bits = &thrd_el[thrd_el.len()-2..thrd_el.len()];
        cell.key = first_two_bits == "11";
        cell.exit = last_two_bits == "11";
        maze.add_cell(cell);

        row +=1;
    }

}


fn main() {
    let mut maze= Maze::new();
    fill_maze(&mut maze);
    println!("{:#?}",maze);
}
