rm *.profraw
CARGO_INCREMENTAL=0 LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
rm -r target/coverage/html/
grcov . --binary-path ./target/debug/deps/ -s . -t html --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html
rm *.profraw
