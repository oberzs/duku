use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub(crate) fn watch_file(path: impl AsRef<Path>, pointer: u32, sender: Sender<(u32, PathBuf)>) {
    let path = path.as_ref().to_owned();

    thread::spawn(move || {
        let file = File::open(&path).unwrap();
        let mut last_modified = None;

        loop {
            let metadata = file.metadata().unwrap();
            let modified = metadata.modified().unwrap();
            if let Some(m) = last_modified {
                if m != modified {
                    sender.send((pointer, path.clone())).unwrap();
                }
            }
            last_modified = Some(modified);
            thread::sleep(Duration::from_millis(500));
        }
    });
}
