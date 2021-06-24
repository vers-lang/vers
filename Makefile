all: compiler

compiler:
	@ $(MAKE) -C versc/ all

dev: all
	@ mv versc/target/debug/versc example/versc
