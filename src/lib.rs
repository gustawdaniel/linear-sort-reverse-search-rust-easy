use std::io::{Error, Read, Write};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut output: Vec<u8> = Vec::new();

        reverse_search(&mut "5 1
1 2 3 4 1".as_bytes(), &mut output).unwrap();
        assert_eq!(&output, b"5\n");
    }


    #[test]
    fn not_found_test() {
        let mut output: Vec<u8> = Vec::new();

        reverse_search(&mut "5 7
1 2 3 4 1".as_bytes(), &mut output).unwrap();
        assert_eq!(&output, b"-1\n");
    }
}

pub fn reverse_search(
    handle: &mut impl Read ,
    output: &mut impl Write,
) -> Result<(), Error> {
    let mut buffer = "".to_string();
    let mut out = "".to_string();
    handle.read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();

    let mut some_line = match lines.next() {
        Some(line) => line,
        _ => ""
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
        Some(line) => line,
        _ => ""
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
            out = format!("{}\n", len);
            break;
        }
        len -= 1;
    }

    if len == 0 {
        out = format!("-1\n");
    }

    output.write_all(out.to_uppercase().as_bytes())?;

    Ok(())
}