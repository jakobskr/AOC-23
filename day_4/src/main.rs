extern crate helpers;

fn main() {
    println!("Hello, world!");
    let path : String = helpers::abs_path() + "/inputs/in";

    let problem : Vec<String> = helpers::input_to_vec(&path, false);

    println!("part a: {}", part_a(&problem));
    println!("part p: {}", part_b(&problem));

}




fn part_a(problem : &Vec<String>) -> i32 {

    let mut total : i32 = 0;
    
    for line in problem {
        let mut l : Vec<&str> = line.split(":").collect();
        // println!("l: {:?}", l);

        l = l[1].split("|").collect();
        // println!("l: {:?}", l);
        // println!("l[0]: {:?}", l[0].trim());
        // println!("l[1]: {:?}", l[1].trim());

        let card :  Vec<i32> = l[0].trim().split_whitespace().map(|x: &str|  x.parse().unwrap()).collect();
        let winnings : Vec<i32> = l[1].trim().split_whitespace().map(|x: &str|  x.parse().unwrap()).collect();

        // println!("card: {:?}", card);
        // println!("winnings: {:?}", winnings);

        let price = winnings.iter().fold(0.5, |acc, x| if card.contains(x) {acc * 2.0} else {acc});

        if price >= 1.0 {
            total += price as i32
        }
    } 

    return total;
}


fn part_b(problem : &Vec<String>) -> i32 {
    let mut total: i32 = 0;

    let mut cards_to_scratch : Vec<i32> = vec![1;problem.len()]; 
    let mut index: usize = 0;
    for line in problem {
        let mut l : Vec<&str> = line.split(":").collect();
        // println!("l: {:?}", l);

        l = l[1].split("|").collect();
        // println!("l: {:?}", l);
        // println!("l[0]: {:?}", l[0].trim());
        // println!("l[1]: {:?}", l[1].trim());

        let card :  Vec<i32> = l[0].trim().split_whitespace().map(|x: &str|  x.parse().unwrap()).collect();
        let winnings : Vec<i32> = l[1].trim().split_whitespace().map(|x: &str|  x.parse().unwrap()).collect();

        // println!("card: {:?}", card);
        // println!("winnings: {:?}", winnings);

        let price: i32 = winnings.iter().fold(1, |acc, x| if card.contains(x) {acc + 1} else {acc});

        let mut i : usize = index + 1;
        while i < index + price as usize && i < cards_to_scratch.len() as usize {
            cards_to_scratch[i] += cards_to_scratch[index];
            i += 1;
        }

        index += 1;
    } 

    return cards_to_scratch.iter().fold(0, |acc, num| acc + num);
}