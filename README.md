AOC 2020
========

Project structure is a little weird. In order to keep everything runnable,
solution execution is managed by examples, with the following directory
structure:
```
examples/
   001_<name>_/        # -> day 1
   002_<name>_/        # -> day 2
   002_<name>_/mem/... # -> day 2 memory examples (running these easily is WIP)
   ...
```

Supporting library code lives in `src/` as would be standard for a rust library.
This allows for testing implementations with `cargo test` and such while still
providing a convenient way to execute specific solutions via the supplied `run`
script with `./run 001` or by executing `cargo run --example=<day>_<name>`
directly.
