use std::env;
use std::process;
use std::io::{ Write };
use instaget:: { argument, download };

fn main() {
    let args: Vec<String> = env::args().collect();

    let argument = argument::Argument::parse(&args);
    if argument.help {
        let h = argument.show_help;
        h();
        process::exit(1);
    }

    if argument.version {
        let v = argument.show_version;
        v();
        process::exit(1);
    }

    if argument.error {
        let e = argument.show_error;
        e();
        process::exit(1);
    }

    let mut out_buffer: Vec<u8> = Vec::new();

    let mut file_type = String::new();

    if let Err(e) = download(&argument.url.unwrap(), &mut out_buffer, &mut file_type) {
        println!("{}", e);
        process::exit(1);
    }

    let mut out_file = match std::fs::File::create(format!("out{}", file_type)) {
        Ok(out_file) => out_file,
        Err(_) => {
            println!("error create output file");
            process::exit(1);
        }
    };

    if let Err(e) = out_file.write_all(&out_buffer[..]) {
        println!("{}", e);
        process::exit(1);
    }
    
}