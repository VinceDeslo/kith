debug:
	RUST_LOG=debug cargo run src/main.rs

run:
	cargo run src/main.rs

set-env:
	export KITH_TSH_PROXY="snyk.teleport.sh:443" && export KITH_TSH_CLUSTER="snyk.teleport.sh"
