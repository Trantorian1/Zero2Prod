.PHONY: watch
watch:
	@cargo watch -x check -x run

.PHONY: test
test:
	@act push
