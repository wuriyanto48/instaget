use crate:: { VERSION };
use url::{ Url };

pub struct Argument {
    pub url: Option<Url>,
    pub unique_path: Option<String>,
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
            unique_path: None,
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

            if flag_param.contains("-h") || 
                flag_param.contains("--help") && args.len() == 2 {
                argument.help = true;
            } else if flag_param.contains("-v") || 
                flag_param.contains("-version") && args.len() == 2 {
                argument.version = true;
            } else {
                let mut out_path = String::new();
                if args.len() == 4 {
                    let output_flag = args[2].clone();
                    let output_path = args[3].clone();
                    if output_flag.contains("-o") || 
                        output_flag.contains("--output") {
                            if let Ok(md) = std::fs::metadata(output_path.clone()) {
                                if md.is_dir() {
                                    out_path = output_path;
                                } else {
                                    argument.error = true;
                                    argument.show_error = Box::new(|| { 
                                        println!("error parsing output path: path is not a directory"); 
                                    });
                                }
                            } else {
                                argument.error = true;
                                argument.show_error = Box::new(|| { 
                                    println!("error parsing output path: path is invalid"); 
                                });
                            }
                    }
                }

                let base_path = std::path::Path::new(&out_path);

                if let Ok(mut parsed_url) = Url::parse(&flag_param) {
                    parsed_url.query_pairs_mut().append_pair("__a", "1");
                    
                    if parsed_url.path_segments().is_none() {
                        argument.unique_path = Some(String::from(base_path.join("out").to_str().unwrap()));
                    } else {
                        let path_segments = parsed_url.path_segments().unwrap();
                        let path_vec: Vec<&str> = path_segments.collect();
                        if path_vec.get(1).is_none() {
                            argument.unique_path = Some(String::from(base_path.join("out").to_str().unwrap()));
                        } else {
                            argument.unique_path = Some(String::from(base_path.join((*path_vec.get(1).unwrap()).to_string()).to_str().unwrap()));
                        }
                    }

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
    println!("----------- instaget usage: -----------");
    println!();
    println!("> instaget https://instagram.com/p/BlaBla");
    println!();
    println!();
    println!("> instaget https://instagram.com/p/BlaBla --output /Users/johndoe/folder");
    println!();
    println!("----------- flag options: -----------");
    println!();
    println!("> -o | --output: set custom output folder");
    println!("> -h | --help: show help");
    println!("> -v | --version: show version");
    println!();
    println!("---------------------------------------");
    println!();
    show_version();
    println!();
    println!("---------------------------------------");
}

fn show_version() {
    println!("instaget version: {}", VERSION);
}

fn show_error() {
    println!("instaget error..");
}