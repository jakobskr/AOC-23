use std::{fs};
use std::env::current_dir;

pub fn abs_path() -> String {
    
    return current_dir().unwrap().into_os_string().into_string().unwrap().to_string();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/*
    input is which day / test
*/
pub fn input_to_vec(input: &str, verbose: bool) -> Vec<String> {
    if verbose {
        println!("reading file: {}" , input);
    }
    
    let mut file_vec : Vec<String> = Vec::new();
    
    for line in fs::read_to_string(input).unwrap().lines() {
        if verbose{
            println!("{}", line);
        }
        file_vec.push(line.to_string());
    }

    return file_vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
