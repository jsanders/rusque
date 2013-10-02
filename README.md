# Rusque

[Resque](https://github.com/resque/resque) compatible worker library for Rust. Inspired by [goworker](https://github.com/benmanns/goworker).

## Disclaimer

This basically works, with the caveat that a job is currently just an unparsed string of json.

## Installation

You can use `rustpkg` to install `rusque`:

```sh
$ rustpkg install github.com/jsanders/rusque
```

## Usage

Import `rusque`:

```rust
extern mod rusque;
```

Create a worker function:

```rust
fn basic_worker(job: rusque::Job) -> rusque::Result {
  println!("I've been asked to work on {:s}", job.job);
  rusque::Ok
}
```

Register the worker to handle a class of jobs:

```rust
let mut rusque = rusque::new();
rusque.register(~"basic_queue", basic_worker);
```

Now work on jobs:

```rust
rusque.work()
```

This will block until an error occurs.

See the whole thing in action! Run:

```sh
$ cd examples/basics
$ rustpkg install github.com/jsanders/rusque
$ ruby basics.rb
$ rust run basics.rs
I've been asked to work on {"class":"Basic","args":[]}
I've been asked to work on {"class":"Basic","args":[]}
I've been asked to work on {"class":"Basic","args":[]}
```
