#![cfg(feature = "profiler")]

use lazy_static::lazy_static;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use std::thread;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

lazy_static! {
    pub static ref PROFILER: Mutex<Profiler> = Mutex::new(Profiler {
        session: None,
        output: None,
        profile_count: 0,
    });
}

pub struct Profiler {
    session: Option<&'static str>,
    output: Option<File>,
    profile_count: u32,
}

pub struct ProfileTimer {
    name: &'static str,
    stopped: bool,
    start_time: SystemTime,
}

struct ProfileResult {
    name: &'static str,
    start: u128,
    end: u128,
    thread_id: u64,
}

impl ProfileTimer {
    pub fn new(name: &'static str) -> Self {
        Self {
            start_time: SystemTime::now(),
            stopped: false,
            name,
        }
    }

    pub fn stop(&mut self) {
        let start = self
            .start_time
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();
        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();

        PROFILER.lock().unwrap().write_profile(ProfileResult {
            name: self.name,
            thread_id: thread::current().id().as_u64().get(),
            start,
            end,
        });

        self.stopped = true;
    }
}

impl Profiler {
    pub fn begin(&mut self, name: &'static str, path: impl AsRef<Path>) -> io::Result<()> {
        self.output = Some(OpenOptions::new().write(true).create(true).open(path)?);
        self.session = Some(name);
        self.write_header();

        Ok(())
    }

    pub fn end(&mut self) {
        self.write_footer();
        self.output = None;
        self.session = None;
        self.profile_count = 0;
    }

    fn write_profile(&mut self, result: ProfileResult) {
        if let Some(output) = self.output.as_mut() {
            if self.profile_count > 0 {
                writeln!(output, ",").unwrap();
            }
            self.profile_count += 1;
            writeln!(
                output,
                "{{\"cat\": \"function\",\"dur\": {}, \"name\": \"{}\", \"ph\": \"X\", \"pid\": 0, \"tid\": {}, \"ts\": {}}}",
                result.end - result.start,
                result.name,
                result.thread_id,
                result.start
            ).unwrap();
        }
    }

    fn write_header(&self) {
        writeln!(
            self.output.as_ref().unwrap(),
            "{{\"otherData\": {{}}, \"traceEvents\": ["
        )
        .unwrap();
    }

    fn write_footer(&self) {
        writeln!(self.output.as_ref().unwrap(), "]}}").unwrap();
    }
}

impl Drop for ProfileTimer {
    fn drop(&mut self) {
        if !self.stopped {
            self.stop();
        }
    }
}
