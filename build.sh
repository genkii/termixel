RUSTFLAGS="-Zlocation-detail=none -Zfmt-debug=none -Zunstable-options" cargo +nightly build \
  -Z build-std=std,panic_abort \
  -Z build-std-features="optimize_for_size" \
  --release

strip --strip-all target/release/termixel
