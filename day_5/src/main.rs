use std::fmt::Debug;


extern crate helpers;

fn main() {
    let path = helpers::abs_path() + "/inputs/in";
    let problem = helpers::input_to_vec(&path, false);
    println!("part a: {}", part_a(&problem));
    println!("part b: {}", part_b(&problem));

}


// if in >= src_start && in <= src_end:
// then out = dst_start + x - src_start

#[derive(Copy, Clone, Debug)]
struct Rule {
    src : u64,
    dst : u64,
    range : u64
}

fn part_a(problem : &Vec<String>) -> u64 {

    let mut all_rules : Vec<Vec<Rule>> = Vec::new();

    let seeds : Vec<u64> = problem[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{:?}", seeds);

    let mut rule_set : Vec<Rule> = Vec::new();
    for line in problem[2..].iter() {
        if line.is_empty() {
            //println!("{:?}", &rule_set);
            all_rules.push(rule_set);
            rule_set = Vec::new();
            continue;
        }

        if line.chars().next().unwrap().is_numeric() {
            let l : Vec<&str> = line.split_whitespace().collect();
            let r = Rule {
                src : l[1].parse().unwrap(),
                dst : l[0].parse().unwrap(),
                range : l[2].parse().unwrap()
            };
            rule_set.push(r);
        }

    }

    all_rules.push(rule_set);

    let mut locations : Vec<u64> = Vec::new();
    for seed in seeds {
        let mut n = seed;
        for rules in &all_rules {
            n = look_up(n, &rules);
        }
        locations.push(n);
    }
    
    return *(locations.iter().min().unwrap());
}

fn part_b(problem : &Vec<String>) -> u64 {

    let mut all_rules : Vec<Vec<Rule>> = Vec::new();

    let seeds : Vec<u64> = problem[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{:?}", seeds);

    let mut rule_set : Vec<Rule> = Vec::new();
    for line in problem[2..].iter() {
        if line.is_empty() {
            all_rules.push(rule_set);
            rule_set = Vec::new();
            continue;
        }

        if line.chars().next().unwrap().is_numeric() {
            let l : Vec<&str> = line.split_whitespace().collect();
            let r = Rule {
                src : l[1].parse().unwrap(),
                dst : l[0].parse().unwrap(),
                range : l[2].parse().unwrap()
            };
            rule_set.push(r);
        }

    }

    all_rules.push(rule_set);

    let mut locations : Vec<u64> = Vec::new();

    let mut i = 0;

    while i < seeds.len()  {
        println!("iteration {} of {}", i / 2, seeds.len() / 2);
        let mut j = 0;
        while seeds[i] + j < seeds[i] + seeds[i + 1] {
            let mut n = seeds[i] + j;
            for rules in &all_rules {
                n = look_up(n, &rules);
            }
            locations.push(n);
            j += 1;
        }

        i += 2
    }
    
    println!("iters: {}", locations.len());
    return *(locations.iter().min().unwrap());
}

fn look_up (n : u64, rules : &Vec<Rule>) -> u64 {

    for rule in rules {
        if n >= rule.src && n < rule.src + rule.range {
            return rule.dst + n - rule.src
        }
    }

    return n;
}



