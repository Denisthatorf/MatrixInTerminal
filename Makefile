CC=clang
CFLAGS=-g -Wall
OBJS=main.o matrix.o ui.o

all: matrix

matrix: $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^ -lncurses
	rm -f $(OBJS)

%.o: src/%.c 
	$(CC) $(CFLAGS) -c $^ -o $@

clean:
	rm -rf *.o main