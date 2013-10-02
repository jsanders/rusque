#[desc = "Resque workers for Rust."];
#[license = "MIT"];

use std::rt::io::timer;

pub mod redis;

pub struct Job {
  job: ~str,
}

pub enum Result {
  Ok,
  Err
}

struct Worker {
  queues: ~[~str],
  worker: ~fn(Job) -> Result,
  redis: redis::Client
}

impl Worker {
  /// Take a list of queues to watch and a function to execute when a job is
  /// received on any of those qeueus.
  pub fn new(queues: ~[~str], worker: ~fn(Job) -> Result) -> Worker {
    Worker {
      queues: queues,
      worker: worker,
      redis: redis::Client::connect()
    }
  }

  /// Look for work in watched queues every second. Block until error.
  pub fn work(&mut self) {
    loop {
      match self.reserve() {
        Some(job) => { (self.worker)(job); },
        None      => timer::sleep(1000)
      }
    }
  }

  /// Return the first job found on any watched queue, or None if none available.
  fn reserve(&mut self) -> Option<Job> {
    for queue in self.queues.iter() {
      let full_queue_name = format!("resque:queue:{:s}", *queue);
      match self.redis.lpop(full_queue_name) {
        Some(job) => {
          return Some(Job { job: job })
        },
        None => continue
      }
    }
    None
  }
}
