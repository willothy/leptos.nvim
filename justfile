build:
  cargo build --release
  cp ./target/release/libleptos_test.so ./lua/leptos.so
