SHELL=bash
default : jki.bin ijk.bin ikj.bin timing.bin
fmt :
	clang-format -i *.c
run : jki.bin ijk.bin ikj.bin
	time ./jki.bin
	time ./ijk.bin
	time ./ikj.bin


%.bin : %.c
	clang -DN=2000 -O3 -o $@ $*.c
