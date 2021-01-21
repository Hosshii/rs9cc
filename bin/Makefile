CC = cc
SRCS=$(wildcard *.c)
RS9CC=../target/debug/rs9cc

all: test.exe extern.exe

tmp_extern.c: extern.c
	$(CC) -o tmp_extern.c -E -P -C extern.c

tmp_test.c: test.c
	$(CC) -o tmp_test.c -E -P -C test.c

common: common.c
	$(RS9CC) common.c > tmp_common.s

test.exe: common tmp_test.c
	$(RS9CC) tmp_test.c > tmp_test.s
	$(CC) -o test.exe -no-pie tmp_common.s tmp_test.s

extern.exe: common tmp_extern.c
	$(RS9CC) tmp_extern.c > tmp_extern.s
	$(CC) -o extern.exe -no-pie tmp_common.s tmp_extern.s

clean:
	rm -rf tmp* *exe