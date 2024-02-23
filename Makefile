EXEC_PATH ?= scripts/example-001.rn

exec:
	@cargo build
	@cp target/debug/galdr ~/.bin
	@$(EXEC_PATH)