set -e
cargo build
cargo build --release
cd ..
cp rust/target/debug/librust.so godot/addons/kdnav/debug/librust.so
cp rust/target/release/librust.so godot/addons/kdnav/release/librust.so
