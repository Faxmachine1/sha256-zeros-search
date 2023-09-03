use clap::Parser;

mod utils;

#[derive(Parser)]
struct Args {
    #[arg(short)]
    n: usize,

    #[arg(short)]
    f: u8,
}

fn main() {
    let cli_args = Args::parse();
    let desired = "0".repeat(cli_args.n);
    
    let mut i = 0;
    let mut found = 0;

    while found < cli_args.f {
        let hash = utils::generate_sha256(i);
        
        if hash.ends_with(&desired) {
            println!("{}, {:?}", i, &hash[7..]);
            found += 1
        }

        i += 1;
    }
}
