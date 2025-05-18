.PHONY: watch
watch:
	@cargo watch -x check -x test -i "tests/**"

.PHONY: test
test:
	@act push
