hello:
	mkdir -p bin
	nasm -felf64 hello.asm -o bin/hello.o
	ld -o bin/hello bin/hello.o
	chmod u+x bin/hello

reg_out:
	mkdir -p bin
	nasm -felf64 reg_out.asm -g -o bin/reg_out.o
	ld -o bin/reg_out reg_out.o
	chmod u+x bin/reg_out

print_call:
	nasm -felf64 print_call.asm -g -o bin/print_call.o
	ld -o bin/print_call print_call.o
	chmod u+x bin/print_call

clean:
	rm -rf bin

