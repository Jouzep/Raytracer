##
## EPITECH PROJECT, 2023
## B-OOP-400-PAR-4-1-raytracer-vincent.shao
## File description:
## Makefile
##

LIB = Sphere_Primitive.so

all:
	cargo build
	cp target/debug/lib$(LIB)  ../../../../plugins/primitives/$(LIB)
clean:
	cargo clean

fclean: clean
	rm -f ../../../../plugins/primitives/$(LIB)
