CC = cc
SRCS=$(wildcard *.c)
RS9CC=../target/debug/rs9cc

all: test.exe extern.exe

common: common.c
	$(RS9CC) common.c > tmp_common.s

test.exe: common test.c
	$(RS9CC) test.c > tmp_test.s
	$(CC) -o test.exe -no-pie tmp_common.s tmp_test.s

extern.exe: common extern.c
	$(RS9CC) extern.c > tmp_extern.s
	$(CC) -o extern.exe -no-pie tmp_common.s tmp_extern.s

clean:
	rm -rf tmp* *exe