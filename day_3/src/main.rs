use std::vec;
use std::cmp::{max, min};

fn main() {

    let path = helpers::abs_path() + "/inputs/in";
    let problem = helpers::input_to_vec(&path, false);

    println!("part_a: {}",part_a(&problem));
    println!("part_b: {}",part_b(&problem));


}


#[derive(Copy, Clone, Debug)]
struct number {
    x : usize,
    y : usize,
    val : u32,
    len : usize
}


fn part_a(problem : &Vec<String>) -> u32 {

    let x = problem[0].len() ;
    let y = problem.len();

    let mut grid : Vec<Vec<char>> = Vec::new();
    let mut numbers : Vec<number> = Vec::new();

    let mut i : usize = 0; let mut j : usize = 0;

    while i < y {
        grid.push(problem[i].chars().collect::<Vec<char>>());
        j = 0;
        while j < x {
            let s = j;
            let line_arr: Vec<char> = problem[i].chars().collect::<Vec<char>>();
            let mut numb : String = "".to_string();
            while j < x && line_arr[j].is_numeric() {
                numb += &line_arr[j].to_string();
                j += 1;
            }         

            if !numb.is_empty() {
                numbers.push( number {x : s, y :i, val : numb.parse().unwrap(), len : numb.len()});
            }

            j += 1;
        }
        i += 1;
    }
    
    // println!("x: {}, y:{}", x ,y);
    // println!("{:?}", numbers);
    return numbers.iter().filter(|n: &&number| symbol_neighbours(*n, &grid, y, x)).fold(0, |acc, n| acc + n.val);
}

fn symbol_neighbours(n : &number, grid : &Vec<Vec<char>>, maxy : usize, maxx : usize) -> bool {

    // check over
    if n.y != 0 {
        let mut s = max((n.x as isize - 1) as isize, 0) as usize;
        let end = min(n.x + n.len + 1, maxx);

        while s < end {
            if !(grid[n.y-1][s].is_numeric() || grid[n.y-1][s] == '.') {
                return true;
            }
            s += 1;
        }
    }
    
    //check under
    if n.y != maxy -1 {
        let mut s = max((n.x as isize - 1) as isize, 0 as isize) as usize;
        let end = min(n.x + n.len + 1, maxx);

        while s < end {
            if !(grid[n.y+1][s].is_numeric() || grid[n.y+1][s] == '.') {
                return true;
            }
            s += 1;
        }
    }

    //check left
    if n.x != 0 {
        if !(grid[n.y][n.x - 1].is_numeric() || grid[n.y][n.x -1] == '.') {
            return true;
        }
    }
    
    if n.x  + n.len < maxx {
        if !(grid[n.y][n.x + n.len].is_numeric() || grid[n.y][n.x + n.len]  == '.') {
            return true;
        }
    }
    
    
    return false;
}

fn part_b(problem : &Vec<String>) -> u32 {
    return 0;
}