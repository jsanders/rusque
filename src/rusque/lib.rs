#[desc = "Resque workers for Rust."];
#[license = "MIT"];

use std::hashmap::HashMap;
use std::rt::io::timer;

pub mod redis;

pub struct Job {
  job: ~str,
}

pub enum Result {
  Ok,
  Err
}

struct Rusque {
  queues: HashMap<~str, ~[~fn(Job) -> Result]>
}

pub fn new() -> Rusque {
  Rusque { queues: HashMap::new() }
}

impl Rusque {
  pub fn register(&mut self, queue: ~str, worker: ~fn(Job) -> Result) {
    let workers = self.queues.find_or_insert(queue, ~[]);
    workers.push(worker);
  }

  pub fn work(&self) {
    let mut redis = redis::Client::connect();
    loop {
      for queue in self.queues.iter() {
        let (short_queue_name, workers) = queue;
        let full_queue_name = format!("resque:queue:{:s}", *short_queue_name);
        match redis.lpop(full_queue_name) {
          Some(job) => {
            for worker in workers.iter() {
              (*worker)(Job { job: job.clone() });
            }
          },
          None => ()
        }
      }
      timer::sleep(1000);
    }
  }
}
