# Rusque

[Resque](https://github.com/resque/resque) compatible worker library for Rust. Inspired by [goworker](https://github.com/benmanns/goworker).

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
fn some_worker(job: &rusque.Job) -> rusque.Result {
  rusque.Ok
}
```

Register the worker to handle a class of jobs:

```rust
rusque.register("SomeClass", some_worker)
```

Now work on jobs:

```rust
rusque.work()
```

This will block until an error occurs.
