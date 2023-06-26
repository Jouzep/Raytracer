##
## EPITECH PROJECT, 2023
## B-OOP-400-PAR-4-1-raytracer-vincent.shao
## File description:
## Makefile
##

PROGRAM_PATH = src/Raytracer

all: raytracer

PRIMITIVE = *Primitive.so

SPHERE = src/DyLib/Primitives/Sphere
PLANE = src/DyLib/Primitives/Plane

all: plugins raytracer

raytracer:
	$(MAKE) -C $(PROGRAM_PATH)

# plugins:
# 	$(MAKE) -C $(PLANE)
# 	$(MAKE) -C $(SPHERE)

clean:
	$(MAKE) clean -C $(PROGRAM_PATH)
	$(MAKE) clean -C $(SPHERE)
	$(MAKE) clean -C $(PLANE)

fclean:
	$(MAKE) fclean -C $(PROGRAM_PATH)
	$(MAKE) fclean -C $(SPHERE)
	$(MAKE) fclean -C $(PLANE)

re: fclean all

.PHONY: all raytracer plugins clean fclean tests_run re