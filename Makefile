CC=g++
build=build
incl:=-Iinclude/ 
flags=-g $(incl)
src=src
out=out
libs := 
files := $(subst src, build, $(patsubst %.c, %.o, $(wildcard $(src)/*.c)))
$(build)/%.o: $(src)/%.c
	$(CC) -c $(flags) -o $(build)/$*.o $(src)/$*.c $(libs)
$(build)/$(out): $(files) $(libs) $(build)
	$(CC) $(flags) -o $(build)/$(out) $(filter-out $(build), $^) 
	
$(build):
	mkdir build
.PHONY: clean
clean:
	rm -rf build
.PHONY: run
run:
	./$(build)/$(out)
