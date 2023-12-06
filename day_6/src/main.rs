use std::vec;

extern crate helpers;

fn main() {
    let path : String = helpers::abs_path() + "/inputs/in";
    let problem : Vec<String> = helpers::input_to_vec(&path, false);
    println!("part a: {}", part_a(&problem));
    println!("part p: {}", part_b(&problem));
}




fn part_a(problem :&Vec<String>) -> u64{
    let races : Vec<u64> = problem[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|x| x.parse().unwrap()).collect();
    let records : Vec<u64> = problem[1].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|x| x.parse().unwrap()).collect();
    let num  =  races.len();
    let mut winning_records : Vec<u64> = vec![];
    let mut i = 0;
    
    while i < num {
        println!("i: {} races: {}", i , races[i]);
        winning_records.push((0..races[i]).collect::<Vec<u64>>().iter().map(|x| distance(races[i], *x)).filter(|x: &u64 | *x > records[i]).collect::<Vec<u64>>().len() as u64);
        i += 1;
    }

    return winning_records.iter().fold(1, |acc ,x | x * acc);
}

fn part_b(problem :&Vec<String>) -> u64{
    let race : u64 = problem[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().collect::<Vec<&str>>().join("").parse().unwrap();
    let record : u64 = problem[1].split(":").collect::<Vec<&str>>()[1].split_whitespace().collect::<Vec<&str>>().join("").parse().unwrap();
    return (0..race).collect::<Vec<u64>>().iter().map(|x| distance(race, *x)).filter(|x: &u64 | *x > record).collect::<Vec<u64>>().len() as u64;
}

fn distance(time: u64, held : u64) -> u64 {
    return held * (time - held);
}