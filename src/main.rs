use std::io;
use std::io::{ BufRead};

fn main() -> io::Result<()> {
    let handle = io::stdin();
    let mut lines = handle.lock().lines();

    let mut some_line = match lines.next() {
        Some(line) => line.unwrap(),
        _ => String::new()
    };

    let mut iterator = some_line.split_ascii_whitespace();

    let mut len: usize = match iterator.next() {
        Some(p) => p.trim().parse().expect("can't read"),
        None => 0
    };
    let  needle = match iterator.next() {
        Some(p) => p.trim().parse().expect("can't read"),
        None => 0
    };

    some_line = match lines.next() {
        Some(line) => line.unwrap(),
        _ => String::new()
    };

    let mut vec:Vec<i32> = vec![0; len];

    iterator = some_line.split_ascii_whitespace();

    for n in 0..len {
        vec[n] = match iterator.next() {
            Some(p) => p.trim().parse().expect("can't read"),
            None => 0
        };
    }

    let mut iter = vec.iter().rev();

    while let Some(num) = iter.next() {
        if *num == needle {
            println!("{}", len);
            break;
        }
        len -= 1;
    }

    Ok(())
}

