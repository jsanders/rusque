extern mod rusque;

fn basic_worker(job: rusque::Job) -> rusque::Result {
  println!("I've been asked to work on {:s} with args {:?}", job.class, job.args);
  rusque::Ok
}

#[test]
fn test_everything() {
  let mut client = rusque::Worker::new(~[~"basic_queue"], basic_worker);
  match client.reserve() {
    Some(job) => { basic_worker(job); },
    None => println!("No job!")
  }
}
