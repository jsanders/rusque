extern mod rusque;

fn basic_worker(job: rusque::Job) -> rusque::Result {
  println(fmt!("I've been asked to work on %s with args %?", job.class, job.args));
  rusque::Ok
}

fn main() {
  let mut rusque = rusque::new();
  rusque.register(~"basic_queue", basic_worker);
  rusque.work();
}
