if [ ! -d "keyboards/$1" ]; then
  echo "Keyboard $1 not found"
  exit 1
fi

echo "Building firmware for $1"

cd keyboards/$1

TARGET=$(grep -m 1 '^target *= *' ./.cargo/config.toml | sed 's/target *= *"\(.*\)"/\1/')

echo $TARGET

cargo install cargo-binutils || true
rustup component add llvm-tools-preview || true
rustup target add $TARGET || true

cargo build --release
rustup component add llvm-tools-preview
cargo objcopy --release -- -O binary ../../firmware.bin
cargo objcopy --release -- -O binary ../../firmware.hex
