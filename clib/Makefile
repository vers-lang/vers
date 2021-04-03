OBJECT = g++ -c

all:
	@ $(OBJECT) messages.cpp
	@ $(OBJECT) string_man.cpp

move:
	@ cp -r messages.o ../build/
	@ cp -r string_man.o ../build/

clean:
	@ rm -rf *.o
