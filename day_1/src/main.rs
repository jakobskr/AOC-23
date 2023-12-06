
extern crate helpers;
fn main() {
    let path = helpers::abs_path() + "/inputs/in";
    println!("path: {}", path);
    let problem : Vec<String> = helpers::input_to_vec(&path, false);
    println!("part a: {}", part_a(&problem));
    println!("part b: {}", part_b(&problem));
}

fn part_a(problem : &[String]) -> i32{
    let mut sum : i32 = 0;
    
    for line  in problem {

        let mut cur_sum = 0;

        for c  in line.chars() {
            if c.is_numeric() {
                cur_sum += c.to_digit(10).unwrap() as i32 * 10;
                break;
            }
        }

        for c in line.chars().rev()  {
            if c.is_numeric() {
                cur_sum += c.to_digit(10).unwrap() as i32;
                break;
            }
        }
        sum += cur_sum;
    }

    return sum;
}

fn part_b(problem : &[String]) -> i32{

    let mut sum : i32 = 0;
    
    for line in problem {

        let mut cur_sum = 0;

        let mut i = 0;
        let line_arr : Vec<char> = line.chars().collect();
        while i < line.len() {
            let c = line_arr[i];
            
            if c.is_numeric() {
                cur_sum += c.to_digit(10).unwrap() as i32 * 10;
                break;
            }
            let spelled_num = spelled_number(&line_arr[i..line_arr.len()]);
            if spelled_num > 0 {
                cur_sum += spelled_num as i32 * 10;
                break;
            }

            i+= 1;

        }

        i = line_arr.len()-1;

        while i as i32 >= 0 {
            let c = line_arr[i];
            
            if c.is_numeric() {
                cur_sum += c.to_digit(10).unwrap() as i32;
                break;
            }
            let spelled_num = spelled_number(&line_arr[i..line_arr.len()]);
            if spelled_num > 0 {
                cur_sum += spelled_num as i32;
                break;
            }
            i-= 1;

        }
        
        sum += cur_sum;

    }

    return sum;

}

fn spelled_number(chars : &[char]) -> i32 {
    let as_string : String = String::from_iter(chars);
    if as_string.starts_with("one") {
        return 1;
    }
    else if as_string.starts_with("two") {
        return 2;
    }
    else if as_string.starts_with("three") {
        return 3;
    }
    else if as_string.starts_with("four") {
        return 4;
    }
    else if as_string.starts_with("five") {
        return 5;
    }
    else if as_string.starts_with("six") {
        return 6;
    }
    else if as_string.starts_with("seven") {
        return 7;
    }
    else if as_string.starts_with("eight") {
        return 8;
    }
    else if as_string.starts_with("nine") {
        return 9;
    }

    return 0;
}