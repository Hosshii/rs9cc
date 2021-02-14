CC = cc
SRCS=$(wildcard *.c)
MAKEFILE_DIR:=$(dir $(abspath $(lastword $(MAKEFILE_LIST))))
PARENT_DIR := $(shell dirname ${MAKEFILE_DIR})
RS9CC=$(PARENT_DIR)/target/debug/rs9cc

all: test.exe extern.exe variadic.exe

common: common.c
	$(RS9CC) common.c > tmp_common.s

test.exe: common test.c
	$(RS9CC) test.c > tmp_test.s
	$(CC) -o test.exe -no-pie tmp_common.s tmp_test.s

extern.exe: common extern.c
	$(RS9CC) extern.c > tmp_extern.s
	$(CC) -o extern.exe -no-pie tmp_common.s tmp_extern.s

variadic.exe: common variadic.c
	$(RS9CC) variadic.c > tmp_variadic.s 
	$(CC) -o variadic.exe -no-pie tmp_common.s tmp_variadic.s 

clean:
	rm -rf tmp* *exe