errFlags = -pedantic -Wall -Wextra -Wcast-align -Wcast-qual -Wctor-dtor-privacy -Wdisabled-optimization -Wformat=2 -Winit-self -Wlogical-op -Wmissing-declarations -Wmissing-include-dirs -Wnoexcept -Wold-style-cast -Woverloaded-virtual -Wredundant-decls -Wsign-conversion -Wsign-promo -Wstrict-null-sentinel -Wstrict-overflow=5 -Wswitch-default -Wundef -Wno-unused

run: build
	./out/main

test: debug
	gdb ./out/main

build: out/
	g++ $(errFlags) -std=gnu++20 -O3 -o out/main libraries/main.cpp 

debug: out/
	g++ $(errFlags) -g -std=gnu++20 -o out/main libraries/main.cpp

out/:
	mkdir out