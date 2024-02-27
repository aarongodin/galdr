EXEC_PATH ?= scripts/example.rn

exec:
	@cargo build
	@cp target/debug/galdr ~/.bin
	@$(EXEC_PATH)

run:
	@$(EXEC_PATH)
