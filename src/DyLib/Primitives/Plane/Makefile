##
## EPITECH PROJECT, 2023
## B-FUN-400-PAR-4-1-wolfram-joseph.yu
## File description:
## Makefile
##

LIB = Plane_Primitive.so

all:
	cargo build
	cp target/debug/lib$(LIB) ../../../../plugins/primitives/$(LIB)

clean:
	cargo clean

fclean: clean
	rm -f ../../../../plugins/primitives/$(LIB)
