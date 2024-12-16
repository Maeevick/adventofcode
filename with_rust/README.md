# Think Stupid

## Run the code

```shell
cargo run
```

## Run the test suite

```shell
cargo test
```

## Run the formatter and the linter
```shell
cargo fmt && cargo clippy
```

## HOW TO (_it's more for me than real guidelines for anyone else_)

- Add one file `dayX.rs` in the `src` folder
- Colocate the code and the test for each part in the file.

```rust
pub fn part1() -> &'static str {
    "This is day XXX, part one"
}

pub fn part2() -> &'static str {
    "This is day XXX, part two"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1solution() {
        assert_eq!(part1(), "This is day XXX, part one");
    }

    #[test]
    fn part2solution() {
        assert_eq!(part2(), "This is day XXX, part two");
    }
}

```
