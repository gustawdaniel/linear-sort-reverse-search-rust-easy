use linear_sort_reverse_search_rust_easy::upcase;
use std::io;

fn main() -> io::Result<()> {
    upcase(&mut io::stdin(), &mut io::stdout())
}

