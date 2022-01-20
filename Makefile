

build:
	cargo build --release

run: 
	cargo run --release

test:
	cargo test --release

clippy:
	cargo clippy -- -D warnings

install:
	wget https://dist.ipfs.io/go-ipfs/v0.11.0/go-ipfs_v0.11.0_linux-amd64.tar.gz && tar -xvf go-ipfs_v0.11.0_linux-amd64.tar.gz && cd go-ipfs/ && sudo sh install.sh
	ipfs init 

start_ipfs:
	tmux
	ipfs init

install_solc:
	wget https://github.com/ethereum/solidity/releases/download/v0.8.11/solc-static-linux && chmod +x solc-static-linux && cp solc-static-linux /usr/local/bin/solc
