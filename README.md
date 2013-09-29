# Rusque

[Resque](https://github.com/resque/resque) compatible worker library for Rust. Inspired by [goworker](https://github.com/benmanns/goworker).

## Disclaimer

This doesn't work yet! Some parts of it pretend like they work, but they don't, really.

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
  println(fmt!("I've been asked to work on %s with args %?", job.class, job.args));
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
