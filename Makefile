.PHONY: watch
watch:
	@RUST_LOG=trace RUST_TEST=1 cargo watch -x "nextest run"

.PHONY: test
test:
	@act push
