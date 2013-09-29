#[desc = "Resque workers for Rust."];
#[license = "MIT"];

use std::hashmap::HashMap;

pub struct Job {
  class: ~str,
  args: ~[~str]
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
    println("registering...");
    let workers = self.queues.find_or_insert(queue, ~[]);
    workers.push(worker);
  }

  pub fn work(&self) {
    println("working...");
    for queue in self.queues.iter() {
      let (queue_name, workers) = queue;
      println(fmt!("found %u workers for %s", workers.len(), *queue_name));
      for worker in workers.iter() {
        (*worker)(Job { class: ~"SomeClass", args: ~[ ~"hey", ~"there" ] });
      }
    }
  }
}
