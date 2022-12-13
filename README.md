[![Rust](https://github.com/gustawdaniel/rust-sqlite-file-redis/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/gustawdaniel/rust-sqlite-file-redis/actions/workflows/rust.yml)

[//]: # (https://stackoverflow.com/questions/53136717/errore0554-feature-may-not-be-used-on-the-stable-release-channel-couldnt)

test:

```
cargo test --bin prepare-tree
```

```
cargo run --bin prepare-tree
```

```
METHOD=split cargo run -- hello
```

Prepare new file

```
time cargo run --bin prepare-file -- g_2.txt 2
```

```
cargo install flamegraph
```

T1:

```
time METHOD=split cargo flamegraph --bin rust_sqlite_file_redis -- "=GJm" g_2
```

T2

```
time METHOD=text cargo flamegraph --bin rust_sqlite_file_redis -- "=GJm" g_2
```

Release

```
time METHOD=bin cargo run --release -- "GGGG" g_2
```