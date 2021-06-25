all: compiler examples

compiler:
	@ $(MAKE) -C versc/ all

examples:
	@ versc -e examples/hello.vers
	@ versc -e examples/math.vers
	@ versc -e examples/string.vers

