use clap::{AppSettings, Clap};

mod rand;
mod pj;

#[derive(Clap)]
#[clap(version = "1.0", author = "David Harting <david.harting@hey.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(about="Generate random values")]
    Rand(Rand),
    #[clap(about="Work with package.json files")]
    Pj(Pj),
}

#[derive(Clap)]
struct Rand {
    #[clap(short, long, default_value = "10")]
    length: usize,
}

#[derive(Clap)]
struct Pj {
    #[clap(subcommand)]
    subcmd: PjSubcmd,
}

#[derive(Clap)]
enum PjSubcmd {
    #[clap(about="List scripts from a package.json")]
    Scripts
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Rand(r) => {
            let config = rand::RandConfig { length: r.length };
            println!("{}", rand::generate(&config));
        }
        SubCommand::Pj(pj) => {
            match pj.subcmd {
                PjSubcmd::Scripts => {
                    match pj::extract_scripts_from_package_json() {
                        Ok(scripts) => {
                            for (key, val) in scripts.iter() {
                                println!("{}:\t\t{}", key, val);
                            }
                        },
                        Err(e) => {
                            match e {
                                pj::PackageJsonError::FileNotFound => {
                                    println!("No package.json in curent directory.");
                                },
                                pj::PackageJsonError::UnableToReadFile(io_error) => {
                                    println!("Unable to read file.\n{}", io_error);
                                },
                                pj::PackageJsonError::Parse(json_error) => {
                                    println!("Unable to parse package.json\n{}", json_error);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
