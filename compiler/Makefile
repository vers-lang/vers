OBJECT = g++ -c

all:
	@ $(OBJECT) versl/asm.cpp
	@ $(OBJECT) compile.cpp
	@ $(OBJECT) project.cpp
	@ $(OBJECT) versl/translate.cpp
	@ $(OBJECT) versl/vers.cpp

move:
	@ cp -r asm.o ../build/
	@ cp -r compile.o ../build/
	@ cp -r project.o ../build/
	@ cp -r translate.o ../build/
	@ cp -r vers.o ../build/

clean:
	@ rm -rf *.o
