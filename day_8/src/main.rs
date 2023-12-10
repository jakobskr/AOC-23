use core::num;
use std::{vec, collections::HashMap, path};
use helpers;


fn main() {
    let path = helpers::abs_path() + "/inputs/in";
    let problem : Vec<String> = helpers::input_to_vec(&path , false);
    
    // println!("part_a: {}", part_a(&problem));
    println!("part_b: {}", part_b(&problem));

}

#[derive(Clone, Debug)]
struct Node {
    v : String,
    l : String,
    r : String,
    
}

fn part_a(problem : &Vec<String>) -> u32 {
    let mut num_inst = 0;

    let mut i: usize = 0;

    let insts : Vec<char> = problem[0].chars().collect();

    let mut nodes : HashMap<String, Node> = HashMap::new();

    for line in &problem[2..] {
        
        let line_vec : Vec<&str> = line.split(" = ").collect();

        let src = line_vec[0];
        let dsts : Vec<String> = line_vec[1].replace("(", "").replace(")", "").split(", ").map(|x| x.to_string()).collect();

        nodes.insert(src.to_string() ,Node {
            v: src.to_string(),
            l : dsts[0].to_string(),
            r : dsts[1].to_string()
        });
    }

    // println!("{:?}", nodes.values());

    let mut n : &Node = nodes.get("AAA").unwrap();
    loop {
        // println!("i: {} n:{:?}, {}", i, n, insts[i % insts.len()]);
        
        if n.v == "ZZZ" {
            break;
        }

        if insts[i % insts.len()] == 'L' {
            n = nodes.get(&n.l).unwrap();
        }

        else {
            n = nodes.get(&n.r).unwrap();
        }

        i += 1;
        num_inst += 1;
    }


    return num_inst;
}


fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let tmp_a = a;
        a = b;
        b = tmp_a % b;
    }
    return a;
}

fn find_path_len(mut n : &Node, nodes : &HashMap<String, Node>, insts : &Vec<char>) -> usize   {
    
    let mut i = 1;

    let mut node : &Node = n;

    for inst in insts.iter().cycle() {
        // let inst: char = insts[i % insts.len()];
        // println!("i: {} {:?}",i,node_gen);

        if *inst == 'L' {
            node = &nodes.get(&node.l).unwrap();
        }
        else {
            node = &nodes.get(&node.r).unwrap();
        }
        
        if node.v.ends_with("Z") {
            return i;
        }

        i+= 1;
    }

    return 0;
}

fn part_b(problem: &Vec<String>) -> usize {
    let mut num_inst = 0;

    let mut i: usize = 0;

    let insts : Vec<char> = problem[0].chars().collect();



    let mut nodes : HashMap<String, Node> = HashMap::new();

    for line in &problem[2..] {
        let line_vec : Vec<&str> = line.split(" = ").collect();

        let src = line_vec[0];
        let dsts : Vec<String> = line_vec[1].replace("(", "").replace(")", "").split(", ").map(|x| x.to_string()).collect();

        nodes.insert(src.to_string() ,Node {
            v: src.to_string(),
            l : dsts[0].to_string(),
            r : dsts[1].to_string()
        });
    }

    let node_starts : Vec<&Node> = nodes.keys().filter(|x| x.ends_with("A")).map(|s| nodes.get(s).unwrap()).collect();
    println!("{:?}",node_starts);

    let mut path_len : Vec<usize> = vec![];

    path_len = node_starts.iter().map(|n| find_path_len(n, &nodes, &insts)).collect();
    

    println!(" {:?}",path_len);

    println!("{} {}", usize::MAX, u32::MAX);
    
    return path_len.iter().fold(1, |acc, x| lcm(acc, *x));
}