# Advent of Code Dev Notes

## Day 1

### General problem

Read lines from file, perform same arithmatic operation per-line. Sum results.

### Hangups

- How to read a file?
	- `fs::read_to_string` -> split by newline -> parse to u32
- How to use iterators?
	- Convert read file to `Vec`. Transform via maths. Sum result.
	- `Vec::iter::map` Does `Vec.iter()` have its own type.
- Proper type conversions: `String` to `u32`
