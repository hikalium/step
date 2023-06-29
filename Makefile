SHELL=bash
default : jki.bin ijk.bin ikj.bin timing.bin sparse_access.bin seq_access.bin
run_timing:
	echo "1" | time --format=%E ./timing.bin
	echo "2" | time --format=%E ./timing.bin
	echo "3" | time --format=%E ./timing.bin
	echo "4" | time --format=%E ./timing.bin

run_seq:
	time ./seq_access.bin
	time ./sparse_access.bin

fmt :
	clang-format -i *.c
run : jki.bin ijk.bin ikj.bin
	time ./jki.bin
	time ./ijk.bin
	time ./ikj.bin

clean :
	-rm *.bin

seq_access.bin : seq_access.c
	clang -O0 -o $@ $<
sparse_access.bin : sparse_access.c
	clang -O0 -o $@ $<

%.bin : %.c
	clang -DN=2000 -O3 -o $@ $*.c
