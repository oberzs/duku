use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

use crate::error::Result;

pub(crate) struct ThreadKill {
    sender: Sender<()>,
    receiver: Arc<Mutex<Receiver<()>>>,
}

impl ThreadKill {
    pub(crate) fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    pub(crate) fn receiver(&self) -> Arc<Mutex<Receiver<()>>> {
        self.receiver.clone()
    }

    pub(crate) fn kill(&self) -> Result<()> {
        self.sender.send(())?;
        Ok(())
    }
}
