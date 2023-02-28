set shell := ["nu", "-c"]

watch:
    watch . { cargo run } --glob=**/*.rs

# use watchexec because Nu watch can't kill process to restart it
watchexec:
    watchexec --exts=rs --on-busy-update=restart -- cargo run

run:
    cargo run

test:
    cargo test

watch-tests:
    watch . { cargo test } --glob=**/*.rs

expected_filename := if os_family() == "windows" { "bitline.exe" } else { "bitline" }

build-release:
    cargo build --release
    @$"Build size: (ls target/release/{{expected_filename}} | get size)"

publish-to-local-bin: build-release
    cp target/release/{{expected_filename}} ~/bin/

build-linux-x64:
    cross build --target x86_64-unknown-linux-gnu --release

build-linux-arm64:
    cross build --target aarch64-unknown-linux-gnu --release

build-windows-on-linux:
    cross build --target x86_64-pc-windows-gnu --release
