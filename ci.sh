curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -- -q -y && rustup default stable && cargo install worker-build && worker-build --release
