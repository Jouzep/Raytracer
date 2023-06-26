##
## EPITECH PROJECT, 2023
## B-FUN-400-PAR-4-1-wolfram-joseph.yu
## File description:
## Makefile
##

NAME = raytracer

PATH_TO_ROOT = ../..

all:
	cargo build
	cp target/debug/$(NAME) $(PATH_TO_ROOT)

clean:
	cargo clean

fclean: clean
	rm -f $(PATH_TO_ROOT)/$(NAME)

re: fclean all

.PHONY: all clean fclean re