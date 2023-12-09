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

#[derive(Copy, Clone, Debug)]
struct Gear {
    y : usize,
    x : usize
}

fn number_neighbours(g : &Gear, grid : &Vec<Vec<char>>, maxy : usize, maxx : usize) -> u32 {
    let mut numbers : Vec<String> = Vec::new();
    if g.y != 0 {

        // top clear, check -1 backwards, and +1 forwards for numbers. 
        if grid[g.y - 1][g.x] == '.' {  
            let mut j = max((g.x as isize - 1) as isize, 0) as usize;
            let end: usize = min(g.x + 1, maxx);

            //middle to left    
            if grid[g.y-1][j].is_numeric() {
                let mut tmp: String = "".to_string();
                
                while j < maxx && grid[g.y - 1][j].is_numeric() {
                    tmp += &grid[g.y - 1][j].to_string();
                    j -= 1;
                }
                numbers.push(tmp.chars().rev().collect());
            }
            
            j = g.x + 1;

            //middle to right
            if grid[g.y-1][j].is_numeric() {
                let mut tmp: String = "".to_string();
                
                while j < maxx && grid[g.y - 1][j].is_numeric() {
                    tmp += &grid[g.y - 1][j].to_string();
                    j += 1;
                }
                numbers.push(tmp);
            }

        }

        // check mid i guess.
        else if grid[g.y - 1][g.x].is_numeric(){
            let mut i : usize = g.x;
            let mut tmp : String = String::new();
            
            while i > 0 && grid[g.y - 1][i - 1].is_numeric() {
                i -= 1;
            }

            while i < maxx && grid[g.y - 1][i].is_numeric() {
                tmp += &grid[g.y - 1][i].to_string();
                i += 1;
            }
            numbers.push(tmp);
        }


    }
    
    //check under
    if g.y != maxy -1 {
        if grid[g.y + 1][g.x] == '.' {
            
            
            let mut j = max((g.x as isize - 1) as isize, 0) as usize;

            //middle to left    
            if grid[g.y+1][j].is_numeric() {
                let mut tmp: String = "".to_string();
                
                while j < maxx && grid[g.y + 1][j].is_numeric() {
                    tmp += &grid[g.y + 1][j].to_string();
                    j -= 1;
                }
                numbers.push(tmp.chars().rev().collect::<String>());
            }
            
            j = g.x + 1;

            //middle to right
            if grid[g.y + 1][j].is_numeric() {
                let mut tmp: String = "".to_string();
                
                while j < maxx && grid[g.y + 1][j].is_numeric() {
                    tmp += &grid[g.y + 1][j].to_string();
                    j += 1;
                }
                numbers.push(tmp);
            }
        }

        else if grid[g.y + 1][g.x].is_numeric(){
            let mut i : usize = g.x;
            let mut tmp : String = String::new();
            
            while i > 0 && grid[g.y + 1][i - 1].is_numeric() {
                i -= 1;
            }

            while i < maxx && grid[g.y + 1][i].is_numeric() {
                tmp += &grid[g.y + 1][i].to_string();
                i += 1;
            }

            numbers.push(tmp);
        }
    }

    //check left
    if g.x != 0 {
        if grid[g.y][g.x - 1].is_numeric() {
            let mut j : usize = g.x - 1;
            let mut tmp: String = "".to_string();
            while j as isize >= 0 && grid[g.y][j].is_numeric() {
                tmp += &grid[g.y][j].to_string();
                j -= 1;
            }
            numbers.push(tmp.chars().rev().collect::<String>());
        }
    }
    
    if g.x + 1 < maxx {
        if grid[g.y][g.x + 1].is_numeric() {
            let mut j : usize = g.x + 1;
            let mut tmp: String = "".to_string();
            while j < maxx && grid[g.y][j].is_numeric() {
                tmp += &grid[g.y][j].to_string();
                j += 1;
            }
            numbers.push(tmp);
        }
    }

    if numbers.len() == 2 {
        return numbers[0].parse::<u32>().unwrap() * numbers[1].parse::<u32>().unwrap();
    }

    return 0;

}

fn part_b(problem : &Vec<String>) -> u32 {
    let x = problem[0].len() ;
    let y = problem.len();

    let mut grid : Vec<Vec<char>> = Vec::new();
    let mut gears : Vec<Gear> = Vec::new();

    let mut i : usize = 0; let mut j : usize = 0;

    while i < y {
        grid.push(problem[i].chars().collect::<Vec<char>>());
        j = 0;
        while j < x {
            let line_arr: Vec<char> = problem[i].chars().collect::<Vec<char>>();

            if line_arr[j] == '*' {
                gears.push(Gear {
                    x : j,
                    y : i
                });
            }      
            j += 1;
        }
        i += 1;
    }

    return gears.iter().map(|g: &Gear| number_neighbours(g, &grid, y, x)).fold(0, |acc, x| acc + x);
}