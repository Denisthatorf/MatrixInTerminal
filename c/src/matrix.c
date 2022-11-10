#include "matrix.h"
#include "ui.h"
#include "config.h"

#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <assert.h>
#include <unistd.h>
#include <string.h>

#define RANDOM_PRINTABLE_CHARACTER (33 + (rand()%80))

typedef struct {
	int x,y;
	bool live;
	bool bright;
} drip;

cell matrix[MAX_X][MAX_Y];
drip drips[NUM_DRIPS];

double rand01();

void init_drips() {
	for(int i = 0; i < NUM_DRIPS; i++) {
			drips[i].live = false;
	}
}

void matrix_init() {
	for(int x = 0; x < MAX_X; x++) {
		for(int y = 0; y < MAX_X; y++) {
			matrix[x][y].char_value = 0;
			matrix[x][y].intensity = 0;
		}
	}
	
	init_drips();
}

void fade_n_change_matrix() {
	for(int x = 0; x < MAX_X; x++) {
		for(int y = 0; y < MAX_Y; y++) {
			if(rand01() < PROB_CHANGE || matrix[x][y].char_value == 0) {
				matrix[x][y].char_value = RANDOM_PRINTABLE_CHARACTER;
			}
			if(rand01() < PROB_DIM) {
				if(matrix[x][y].intensity > 0 )
					matrix[x][y].intensity -= 1;
			}
		}
	}
}

void try_add_drips() {
	for(int i = 0; i < NUM_DRIPS; i++) {
		if(drips[i].live == false) {
			drips[i].live = true;
			drips[i].x = rand() % MAX_X;
			drips[i].y = 0; //rand() % MAX_Y
			drips[i].bright = rand() % 2;
			return;
		}
	}
}

void update_drips() {
	for(int i = 0; i < NUM_DRIPS; i++) {
		if(drips[i].live) {
			if(drips[i].bright) {
				matrix[drips[i].x][drips[i].y].intensity = MAX_INTENSITY;
			}
			else {
				matrix[drips[i].x][drips[i].y].intensity = MIN_INTENSITY;
			}
			//rips die when they leave the screen
			if(++drips[i].y >= MAX_Y-1) {
				drips[i].live = false;
			}
		}
	}
}

void matrix_update() {
	if(rand01() < PROB_DRIP_SPAWN) {
		try_add_drips();
	}
	update_drips();

	fade_n_change_matrix();
}


double rand01() {	
	return (double)rand() / (double)RAND_MAX;
}
