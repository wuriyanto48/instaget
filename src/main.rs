use std::env;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::io::{ Write };
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::{ Arc, Mutex };
use instaget:: { argument, download };

fn main() {
    let done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

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

    let shared_file_type = Arc::new(Mutex::new(String::new()));
    let file_type = shared_file_type.clone();

    let (tx_download, rx_download) = mpsc::channel::<Vec<u8>>();
    let (tx_done, rx_done) = mpsc::channel::<bool>();

    let done_c = done.clone();

    // download thread
    thread::spawn(move || {
        let mut out_buffer: Vec<u8> = Vec::new();
        if let Err(e) = download(&argument.url.unwrap(), &mut out_buffer, file_type) {
            println!("{}", e);
            process::exit(1);
        }


        tx_download.send(out_buffer).unwrap();
        
        done_c.store(true, Ordering::Relaxed);
    });

    // loading bar thread
    thread::spawn(move || {
        print!("downloading ");
        loop {
            if done.load(Ordering::Relaxed) {
                break;
            }

            print!(". ");
            std::io::stdout().flush().unwrap();
            thread::sleep(std::time::Duration::from_millis(500));
        }

        println!();

        tx_done.send(true).unwrap();
    });

    if rx_done.recv().unwrap() {
        let data = match rx_download.recv() {
            Ok(data) => data,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        };
        
        let mut out_file = match std::fs::File::create(format!("out{}", 
            *shared_file_type.lock().unwrap())) {
            Ok(out_file) => out_file,
            Err(_) => {
                println!("error create output file");
                process::exit(1);
            }
        };
    
        if let Err(e) = out_file.write_all(&data[..]) {
            println!("{}", e);
            process::exit(1);
        }
    }
    
}