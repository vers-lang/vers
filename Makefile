OBJECT = g++ -c
BINARY = g++ -static -o ../vers

all: setup clibs source vlcompiler bin

setup:
	@ rm -rf build/
	@ mkdir build/

clibs:
	@ echo "Compiling C libraries from clib/..."
	@ $(MAKE) -C clib/ all
	@ $(MAKE) -C clib/ move

source:
	@ echo "Compiling src/..."
	@ $(MAKE) -C src/ all
	@ $(MAKE) -C src/ move

vlcompiler:
	@ echo "Compiling compiler/..."
	@ $(MAKE) -C compiler/ all
	@ $(MAKE) -C compiler/ move

bin:
	@ echo "Creating Vers command..."
	@ cd build/ && $(BINARY) *.o

clean:
	@ rm -rf build/
	@ rm -rf vers
	@ $(MAKE) -C clib/ clean
	@ $(MAKE) -C compiler/ clean
	@ $(MAKE) -C src/ clean

requirements:
	sudo apt-get update
	sudo apt-get upgrade
	sudo apt-get install libboost-all-dev
