#include <stdio.h>
const int TAPE_SIZE = 4096;
const int REF_TAPE_SIZE = 256;

unsigned int tape[4096];
unsigned int ref_tape[4096];
unsigned int ptr = 0;
unsigned int ref_ptr = 0;

unsigned int allocate() {
    unsigned int size = tape[ptr]; 
    int cons_empty_spaces = 0; 
    for (int i=TAPE_SIZE-1; i>0; i--) {
        if (tape[i] == 0) { cons_empty_spaces++; }
        else { cons_empty_spaces = 0; }
        if (cons_empty_spaces == size) { return i; }
    }
    return 0;
}


void plus(int n) {
    tape[ptr] += n;
}

void minus(int n) {
    tape[ptr] -= n;
}

void set(int n) {
    tape[ptr] = n;
}

void left(int n) {
    ptr -= n;
}

void right(int n) {
    ptr += n;
}

void deref() {
    ref_tape[ref_ptr++ % REF_TAPE_SIZE] = ptr;
    ptr = tape[ptr];
}

void refer() {
    ptr = ref_tape[--ref_ptr % REF_TAPE_SIZE];
}

int main() {

set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
left(7);
set(0);
left(1);
set(0);
right(8);
while (tape[ptr]) {
left(8);
plus(1);
right(1);
plus(1);
right(7);
minus(1);
}

left(7);
while (tape[ptr]) {
right(7);
plus(1);
left(7);
minus(1);
}

right(7);
set(0);set(0);
left(8);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
}