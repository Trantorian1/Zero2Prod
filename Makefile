.PHONY: watch
watch:
	@cargo watch -x check -x run -i "tests/**"

.PHONY: test
test:
	@act push
