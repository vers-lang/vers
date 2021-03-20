USER_NAME = $USER

all: clean dirs vers libs move

clean:
	@ echo "Cleaning root directory of Vers files and directorires..."
	@ sudo rm -rf /bin/vers
	@ sudo rm -rf ~/.vers/
	@ sudo rm -rf /lib/vers/
	@ echo "Cleaning project directory..."
	@ rm -rf example/build/
	@ rm -rf lib/std/build/
	@ rm -rf target/
	@ echo "Done"

dirs:
	@ sudo mkdir /lib/vers
	@ sudo mkdir /lib/vers/tools
	@ sudo mkdir /lib/vers/vers
	@ sudo mkdir ~/.vers/

vers:
	@ echo "Compiling the Vers compiler, language, and stdlib..."
	@ cargo build
	@ sudo mv target/debug/vers /bin/vers

libs:
	@ echo "Compiling Vers stdlib"
	@ cd lib/ && make all

move:
	@ echo "Moving binaries and libraries..."
	@ sudo cp -r version /lib/vers/vers/version
