use crate:: { VERSION };
use url::{ Url };

pub struct Argument {
    pub url: Option<Url>,
    pub help: bool,
    pub version: bool,
    pub error: bool,
    pub show_help: fn(),
    pub show_version: fn(),
    pub show_error: Box<dyn Fn() + Send>,
}

impl Argument {
    pub fn parse(args: &[String]) -> Argument {
        let mut argument = Argument{
            url: None,
            help: false,
            version: false,
            error: false,
            show_help,
            show_version,
            show_error: Box::new(show_error)
        };

        if args.len() < 2 {
            argument.help = true;
        } else {
            let flag_param = args[1].clone();

            if flag_param.contains("-h") || flag_param.contains("--help") && args.len() == 2 {
                argument.help = true;
            } else if flag_param.contains("-v") || flag_param.contains("-version") && args.len() == 2 {
                argument.version = true;
            } else {
                if let Ok(mut parsed_url) = Url::parse(&flag_param) {
                    parsed_url.query_pairs_mut().append_pair("__a", "1");
                    argument.url = Some(parsed_url);
                } else {
                    argument.error = true;
                    argument.show_error = Box::new(|| { 
                        println!("error parsing url: url is invalid"); 
                    });
                }
            }
        }

        argument
    }
}

fn show_help() {
    println!();
    println!("instaget usage:");
    println!();
    println!("instaget https://instagram.com/p/BlaBla");
    println!();
    println!("-h/ --help: show help");
    println!("-v/ --version: show version");
    println!();
}

fn show_version() {
    println!("instaget version: {}", VERSION);
}

fn show_error() {
    println!("instaget error..");
}