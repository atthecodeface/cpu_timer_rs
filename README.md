# cpu_timer

A library to support timing execution of code using a high precision,
low overhead CPU clock tick, with a fallback to std::time where the
CPU architecture does not support a high precision timer.

This provides a suite of timer types, from simple elapsed-ticks,
through various accumulation and occurrence counted elapsed timers,
and traces.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cpu-timer = "0.1.0"
```

## Releases

Release notes are available in [RELEASES.md](RELEASES.md).

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
