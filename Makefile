all: compiler

compiler:
	@ $(MAKE) -C versc/ all

dev:
	@ mv versc/target/debug/versc example/versc
