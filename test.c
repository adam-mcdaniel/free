#include <stdio.h>

unsigned int tape[96];
unsigned int ref_tape[256];
unsigned int ptr = 0;
unsigned int ref_ptr = 0;

unsigned int allocate() { 
	unsigned int size = tape[ptr]; 
	int cons_empty_spaces = 0; 
	for (int i=96-1; i>0; i--) {
		if (tape[i] == 0) { cons_empty_spaces++; }
		else { cons_empty_spaces = 0; }
		if (cons_empty_spaces == size) { return i; }
	}
	return 0;
}

int main() {
	
	for(int loop = 0; loop < 96; loop++) printf("%u ", tape[loop]);
printf("\nPOINTER %u\n", ptr);}
