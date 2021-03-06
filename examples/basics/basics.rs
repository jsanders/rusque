extern mod rusque;

fn basic_worker(job: rusque::Job) -> rusque::Result {
  println!("I've been asked to work on {:s} with args {:?}", job.class, job.args);
  rusque::Ok
}

fn main() {
  let mut client = rusque::Worker::new(~[~"basic_queue"], basic_worker);
  client.work();
}
