#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#include <ncurses.h>

#include "ui.h"
#include "matrix.h"
#include "config.h"

int main()
{
	if(!init_ui()) {
		return EXIT_FAILURE;
	}

	matrix_init();

	//for(int i =0; i < ITERATIONS; i++){
	for(;;) {
		matrix_update();
		show_matrix();
		usleep(REFRESH_DELAY);
	}

	cleanup_ui();
	return EXIT_SUCCESS;
}
