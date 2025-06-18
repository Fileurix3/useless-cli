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

    #[arg(short, long)]
    password: Option<Vec<u8>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.calc.is_some() {
        let result: i64 = utils::calc::calc(&args.calc.unwrap());
        println!("Calc result: {}", result);
    }

    if args.xorshift.is_some() {
        utils::xorshift::xorshift(args.xorshift.unwrap())?;
    }

    if args.password.is_some() {
        let data = args.password.unwrap();
        utils::rand_data::rand_password(data[0], data[1])?;
    }

    Ok(())
}
