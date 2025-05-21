.PHONY: watch
watch:
	@RUST_LOG=trace cargo watch -x "nextest run"

.PHONY: test
test:
	@act push
