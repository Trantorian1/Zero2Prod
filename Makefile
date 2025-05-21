.PHONY: watch
watch:
	@cargo watch -x check -x "nextest run"

.PHONY: test
test:
	@act push
