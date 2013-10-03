#[desc = "Resque workers for Rust."];
#[license = "MIT"];

extern mod extra;
use extra::json;

use std::rt::io::timer;

pub mod redis;

pub struct Job {
  class: ~str,
  args: json::List
}

impl Job {
  fn new(json: json::Json) -> Job {
    let format_error = "Invalid json";
    match json {
      json::Object(top) => {
        let class_json = top.find(&~"class").expect(format_error);
        let class = match *class_json {
          json::String(ref class) => class.clone(),
          _ => fail!(format_error)
        };

        let args_json = top.find(&~"args").expect(format_error);
        let args = match *args_json {
          json::List(ref args_list) => args_list.clone(),
          _ => fail!(format_error)
        };

        Job { class: class, args: args }
      },
      _ => fail!(format_error)
    }
  }
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
  pub fn reserve(&mut self) -> Option<Job> {
    for queue in self.queues.iter() {
      let full_queue_name = format!("resque:queue:{:s}", *queue);
      match self.redis.lpop(full_queue_name) {
        Some(job_str) => {
          let job_json = json::from_str(job_str).unwrap();
          return Some(Job::new(job_json))
        },
        None => continue
      }
    }
    None
  }
}
