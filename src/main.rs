use std::env;

fn count_bits(s: &str) -> u64 {
    s.len() as u64 * 8u64
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: bitcount <string>");
        std::process::exit(1);
    }

    let bits = count_bits(&args[1]);
    println!("{}", bits);
}
