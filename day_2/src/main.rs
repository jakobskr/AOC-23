use helpers;
use std::collections::HashMap;
fn main() {

    let path = helpers::abs_path() + "/inputs/in";
    let problem = helpers::input_to_vec(&path, false);

    println!("part_a: {}",part_a(&problem));
    println!("part_b: {}",part_b(&problem));


}


fn part_a(problem : &Vec<String>) -> u32 {
    let mut sum : u32 = 0;
    let max_red: i32 = 12;
    let max_green: i32 = 13;
    let max_blue: i32 = 14;

	

    for line in problem {
		let mut valid : bool =  true;
 
		let line_vec : Vec<&str> = line.split(":").collect();
		let id : u32 = line_vec[0].split_whitespace().collect::<Vec<&str>>()[1].parse().unwrap();

		for round in line_vec[1].split(";") {
			
			let mut cube_map : HashMap<String,i32> = HashMap::from([
				("red".to_string(), max_red), 
				("green".to_string(), max_green),
				("blue".to_string(), max_blue)]);

			for draw in round.split(",") {
				let draw_vals : Vec<&str>= draw.split_whitespace().collect();
				let amount : i32 = draw_vals[0].parse().unwrap();
				let colour = draw_vals[1];
				let cur_amount: &i32 = cube_map.get(colour).unwrap();

				cube_map.insert(colour.to_string(), cur_amount - amount);
			}

			if cube_map.values().any(|x: &i32| *x < 0) {
				valid = false;
				break;
			}
		}
		
		if valid {
			sum += id;
		}	
    }  

    sum
}

fn part_b(problem : &Vec<String>) -> u32 {
	let mut sum : u32 = 0;
	for line in problem {
		let mut cube_map : HashMap<String,i32> = HashMap::from([
			("red".to_string(), 0), 
			("green".to_string(), 0),
			("blue".to_string(), 0)]);
	
		let line_vec : Vec<&str> = line.split(":").collect();

		for round in line_vec[1].split(";") {
			

			for draw in round.split(",") {
				// println!("{}",draw);
				let draw_vals : Vec<&str>= draw.split_whitespace().collect();
				let amount : i32 = draw_vals[0].parse().unwrap();
				let colour = draw_vals[1];
				let cur_amount: &i32 = cube_map.get(colour).unwrap();

				cube_map.insert(colour.to_string(), std::cmp::max(*cur_amount, amount));
			}

		}
		
		sum += cube_map.values().fold(1, |acc: u32, x: &i32| acc * (*x as u32));
	}

	return sum;
}
