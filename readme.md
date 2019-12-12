# aoc2019

My solutions for <https://adventofcode.com/2019/>

## Info

Files for each solution are in their respectively numbered folder

Question markdown is generated using [a userscript](https://github.com/camas/userscripts/blob/master/aoc-markdown.user.js)

Use `solve.py` or `cargo run` to run solutions. Both use the same options

VSCode launch configs are also provided

For each solution `solve.py` calls `solve(data: str)` to run inputs and `tests() -> List[Tuple[str, str]]` to get test data

Similarly, `solve` calls `solve(data: Vec<&str>) -> String` to run inputs and tests are run using `cargo test`

Tests (and the python `tests()` function) are optional

If a single solution is run the answer is copied to clipboard

## Examples

Show help:

- `python solve.py -h`

- `cargo run -- -h`

Run solution 1:

- `python solve.py 1`

- `cargo run -- 1`

Run solutions 1, 2 and 3:

- `python solve.py 1 2 3`

- `cargo run -- 1 2 3`
