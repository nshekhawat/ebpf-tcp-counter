.PHONY: all clean

BPF_CFLAGS = -O2 -g -Wall -Werror

all: tcp_counter.o

tcp_counter.o: tcp_counter.c
	clang $(BPF_CFLAGS) -target bpf -c tcp_counter.c -o tcp_counter.o

clean:
	rm -f *.o
