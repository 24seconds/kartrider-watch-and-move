extern crate fs_extra;
extern crate notify;

use fs_extra::file::{move_file, CopyOptions};
use notify::{op::REMOVE, raw_watcher, RecursiveMode, Watcher};
use std::{path::PathBuf, sync::mpsc::channel};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// THe path to watch
    #[structopt(parse(from_os_str), long)]
    watch_path: PathBuf,

    /// The path to move file to
    #[structopt(parse(from_os_str), long)]
    target_path: PathBuf,
}

fn main() {
    let args = Cli::from_args();

    println!("watchPath: {:?}", args.watch_path);
    println!("targetPath: {:?}", args.target_path);


    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = raw_watcher(tx).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(
            "/Users/young/Desktop/moloco/playground/rust/watcher_sample",
            RecursiveMode::Recursive,
        )
        .unwrap();

    let destination_path = "/Users/young/Desktop/moloco/playground/rust/target_sample";
    let options = CopyOptions::new(); //Initialize default values for CopyOptions

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);

                match event.op {
                    Ok(op) => {
                        // Do nothing
                        if op & REMOVE == REMOVE {
                            continue;
                        }
                    }
                    Err(err) => println!("Error at receiving event!, {:?}", err),
                }

                let source = event.path;

                let source_file_name = source
                    .as_ref()
                    .and_then(|name| name.file_name())
                    .and_then(|name| name.to_str())
                    .unwrap();

                let destination = format!("{}/{}", destination_path, source_file_name);

                println!("source file name: {}", &source_file_name);
                println!("destination: {}", &destination);

                match move_file(source.unwrap(), destination, &options) {
                    Ok(_) => println!("File moved!"),
                    Err(reason) => println!("Failed to move!, {:?}", reason),
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
