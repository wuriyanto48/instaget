use std::env;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::io::{ Write };
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::{ Arc };
use instaget:: { argument, download_to_tx };

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

    // begin to download
    let done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    let (tx_download, rx_download) = mpsc::channel::<Vec<u8>>();
    let (tx_done, rx_done) = mpsc::channel::<bool>();
    let (tx_file_type, rx_file_type) = mpsc::channel::<String>();

    let done_c = Arc::clone(&done);

    // download thread
    thread::spawn(move || {
        if let Err(e) = download_to_tx(&argument.url.unwrap(), 
            tx_download, tx_file_type, done_c) {
            println!("{}", e);
            process::exit(1);
        }
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

    let file_type = match rx_file_type.recv() {
        Ok(file_type) => file_type,
        Err(_) => {
            println!("error getting file type");
            process::exit(1);
        }
    };

    let mut out_file = match std::fs::File::create(format!("out{}", file_type)) {
        Ok(out_file) => out_file,
        Err(_) => {
            println!("error create output file");
            process::exit(1);
        }
    };

    for data in rx_download {
        if let Err(e) = out_file.write(&data[..]) {
            println!("{}", e);
            process::exit(1);
        }
    }

    let download_done = match rx_done.recv() {
        Ok(download_done) => download_done,
        Err(_) => {
            println!("error create output file");
            process::exit(1);
        }
    };

    if download_done {
        println!("download done..");
    }
    
}