# LEETCODE

Solution to leetcode daily challenges in rust.

## Setup.

Clone `cargo-leet` repo and update location in `Cargo.toml` dependencies.

```
git clone https://github.com/rust-practice/cargo-leet
```

## Usage.

To pull challenge code snippets (if slug is emit daily challenge is generated):

```bash
cargo leet generate $slug
```

To run a challenge:

```bash
cargo leet test
```

## FAQ

- what should be the value of slug?
  - If the URL of challenge is: `https://leetcode.com/problems/dota2-senate/`
  - Then the slug is `dota2-senate`
  - Note: ignore description in case of `https://leetcode.com/problems/dota2-senate/description/`
