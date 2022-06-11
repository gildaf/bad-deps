# dev-dependencies bug

This is just an example reproduce the problem we have in cargo

## How to run this

just `cargo run`
It should print "This is ... not a test" but it prints "This is ... a test"

## How to run this (with docker)

```bash
docker build -t baddeps .
```

and then

```bash
docker run baddeps cargo run
```

## The Problem

The Problem is that the `test` flag
(defined in [Cargo.toml](get_str/Cargo.toml) of get_str)
and used in ([Cargo.toml](core/Cargo.toml) of core)
under dev-dependencies should only be used in tests.
But it is actually used in "regular" runs.  