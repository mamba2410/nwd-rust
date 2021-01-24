// PROJECT_NAME.c

#include <stdio.h>
#include <pico/stdlib.h>
#include <pico/binary_info.h>


int main() {
	bi_decl(bi_program_description("PROJECT_NAME"));

	stdio_init_all();

	printf("Hello, World!\n");

	while(1)
		tight_loop_contents();
}
