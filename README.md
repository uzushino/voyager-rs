# voyager-rs

[spotify/voyager](https://github.com/spotify/voyager) bindings for Rust.

```toml
[dependencies]
voyager_rs = { git = "https://github.com/uzushino/voyager_rs" }
```

## Building

```
$ git clone git@github.com:uzushino/voyager-rs.git
$ cd voyager-rs
$ git submodule update
$ cargo test
```

## Example

```rust
let v = Voyager::new();

let v1 = &[1.0, 2.0, 3.0, 4.0, 5.0];
let v2 = &[6.0, 7.0, 8.0, 9.0, 10.0];

v.add_item(v1, Some(1));
v.add_item(v2, Some(2));

let (result, distance) = v.query(v1, 2, None);

assert!(result == vec![1, 2]);
assert!(distance == vec![0.0, 125.0]);
```

## Feature

- [x] add_item
- [x] get_distance
- [x] query