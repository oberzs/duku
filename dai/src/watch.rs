// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// function to watch file for changes

use std::fs::File;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

pub fn watch_file(path: impl AsRef<Path>) -> Receiver<()> {
    let path = path.as_ref().to_owned();
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let file = File::open(&path).expect("bad file");
        let mut last_modified = None;

        loop {
            let metadata = file.metadata().expect("bad metadata");
            let modified = metadata.modified().expect("bad modifier");
            if let Some(m) = last_modified {
                if m != modified {
                    sender.send(()).expect("bad send");
                }
            }
            last_modified = Some(modified);
            thread::sleep(Duration::from_millis(500));
        }
    });

    receiver
}
