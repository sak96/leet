# LEETCODE

Solution to leetcode daily challenges in rust.


## Usage.

To pull challenge code snippets:
```bash
cargo generate $slug
```

To run a challenge:
```bash
cargo test $slug
```

## FAQ

* what should be the value of slug?
  * If the URL of challenge is: `https://leetcode.com/problems/dota2-senate/`
  * Then the slug is `dota2-senate`
  * Note: ignore description in case of `https://leetcode.com/problems/dota2-senate/description/`
