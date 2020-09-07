#![cfg(feature = "glsl")]

use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::pipeline::Shader;
use crate::storage::Handle;

pub(crate) fn watch_file(
    path: impl AsRef<Path>,
    shader: Handle<Shader>,
    sender: Sender<(Handle<Shader>, PathBuf)>,
) {
    let path = path.as_ref().to_owned();

    thread::spawn(move || {
        let file = File::open(&path).expect("bad path");
        let mut last_modified = None;

        loop {
            let metadata = file.metadata().expect("bad metadata");
            let modified = metadata.modified().expect("bad modified");
            if let Some(m) = last_modified {
                if m != modified {
                    sender
                        .send((shader.clone(), path.clone()))
                        .expect("bad receiver");
                }
            }
            last_modified = Some(modified);
            thread::sleep(Duration::from_millis(500));
        }
    });
}
