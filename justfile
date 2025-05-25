all: compile copy

compile:
    cargo build --release --target wasm32-unknown-unknown

copy:
    cp ./target/wasm32-unknown-unknown/release/typst_shapemaker.wasm .


clean:
    cargo clean
    rm -rf target