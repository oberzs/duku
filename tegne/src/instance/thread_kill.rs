use crossbeam_channel::bounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;

use crate::error::Result;

pub(crate) struct ThreadKill {
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl ThreadKill {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = bounded(1);
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
