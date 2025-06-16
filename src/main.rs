use clap::Parser;

mod utils;

#[derive(Parser, Debug)]
#[command(name = "uslc")]
struct Args {
    #[arg(short, long, value_name = "EXPR", help = "Math expression")]
    calc: Option<String>,

    #[arg(
        short,
        long,
        value_name = "NUMBER",
        help = "Random number use XORShift"
    )]
    xorshift: Option<u8>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.calc.is_some() {
        let result = utils::calc::calc(&args.calc.unwrap());
        println!("Calc result: {}", result);
    }

    if args.xorshift.is_some() {
        utils::xorshift::xorshift(args.xorshift.unwrap());
        println!("")
    }

    Ok(())
}
