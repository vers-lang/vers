all: clean deps build

clean:
	@ rm -rf libc/
	@ rm -rf libc.vers.lib

deps:
	@ echo "Compiling Vers LibC..."
	@ git clone https://github.com/vers-lang/libc.git
	@ cd libc/ && make all && mv libc ../libc.vers.lib

build:
	@ echo "Compiling std..."

