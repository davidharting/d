use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "David Harting <david.harting@hey.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
  #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.0")]
    Rand(Rand)
}

#[derive(Clap)]
struct Rand{
    #[clap(short, long, default_value="10")]
    length: u32,
}

fn main() {
    let opts : Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Rand(r) => {
            println!("Ran 'rand' with length {}", r.length)
        }
    }
}
