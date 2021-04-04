OBJECT = g++ -c

all:
	@ $(OBJECT) compile.cpp
	@ $(OBJECT) functions.cpp
	@ $(OBJECT) project.cpp
	@ $(OBJECT) versl/asm.cpp
	@ $(OBJECT) versl/exe.cpp
	@ $(OBJECT) versl/gcc.cpp
	@ $(OBJECT) versl/lib.cpp
	@ $(OBJECT) versl/translate.cpp

move:
	@ cp -r asm.o ../build/
	@ cp -r compile.o ../build/
	@ cp -r exe.o ../build/
	@ cp -r gcc.o ../build/
	@ cp -r lib.o ../build/
	@ cp -r functions.o ../build/
	@ cp -r project.o ../build/
	@ cp -r translate.o ../build/

clean:
	@ rm -rf *.o
