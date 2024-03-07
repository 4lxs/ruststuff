use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'n')]
    no_newline: bool,

    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    print!(
        "{}{}",
        cli.args.join(" "),
        if cli.no_newline { "" } else { "\n" }
    );
}
