use crossbeam::channel;
use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;

use crate::error::Result;

pub(crate) struct ThreadKill {
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl ThreadKill {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = channel::bounded(1);
        Self { sender, receiver }
    }

    pub(crate) fn receiver(&self) -> Receiver<()> {
        self.receiver.clone()
    }

    pub(crate) fn kill(&self) -> Result<()> {
        self.sender.send(())?;
        Ok(())
    }
}
