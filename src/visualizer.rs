use std::fs::{self};
use structs::Map;
mod input;
mod structs;

fn read_map(path: &str) -> Result<String, String> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("{e}")),
    }
}

fn strncmp(s1: &str, s2: &str, n: usize) -> i8 {
    let mut ch1 = s1.chars();
    let mut ch2 = s2.chars();

    for _ in 0..n {
        match (ch1.next(), ch2.next()) {
            (Some(ch1), Some(ch2)) => {
                if ch1 != ch2 {
                    return (ch1 as i8).wrapping_sub(ch2 as i8);
                }
            }
            (Some(ch1), None) => return ch1 as i8,
            (None, Some(ch2)) => return -(ch2 as i8),
            (None, None) => return 0,
        }
    }
    0
}

fn get_first_char(string: &str) -> char {
    match string.chars().next() {
        Some(c) => c,
        None => panic!("Error: Value mustn't be empty."),
    }
}

fn print_map(map: &Map) {
    let mut v: Vec<&str> = map.get_map().split('\n').collect();
    let walls = map.get_walls();
    let paths = map.get_paths();
    let wall = get_first_char(walls);
    let path = get_first_char(paths);

    v.remove(v.len() - 1);
    print!("   ");
    for x in 0..v.len() {
        if x < 10 {
            print!(" 0{x}");
        } else if x < 100 {
            print!(" {x}");
        } else {
            print!("{x}");
        }
    }
    println!();
    for (y, string) in v.iter().enumerate() {
        let mut nb = 0;
        if y < 10 {
            print!("0{y} ");
        } else if y < 100 {
            print!("{y} ");
        } else {
            print!("{y}");
        }
        for (i, c) in string.chars().enumerate() {
            let s1 = string.split_at(i).1;
            if nb != i {
                continue;
            }
            if c == wall && strncmp(s1, walls, walls.len()) == 0 {
                nb += walls.len();
                print!("\x1b[30;47m   ");
            } else if c == path && strncmp(s1, paths, paths.len()) == 0 {
                nb += paths.len();
                print!("\x1b[30;44m   ");
            } else {
                nb += 1;
            }
        }
        println!("\x1b[0m");
    }
}

pub fn visualizer() {
    match read_map(&input::read_input("Enter map file path: ")) {
        Ok(map) => {
            let wall = input::read_input("Enter walls characters: ");
            let path = input::read_input("Enter paths characters: ");
            match Map::new(&map, &wall, &path) {
                Ok(m) => print_map(&m),
                Err(err) => panic!("Error: {err}"),
            }
        }
        Err(err) => println!("Error: {err}"),
    }
}
