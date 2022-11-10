#ifndef UI_H
#define UI_H

#include <stdbool.h>
#include "config.h"

typedef struct cell {
	char char_value;
	int intensity;
} cell;

extern cell matrix[MAX_X][MAX_Y];

bool init_ui();
void cleanup_ui();

void show_matrix();

#endif
