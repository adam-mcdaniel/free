// OPTIMIZED 10512 INSTRUCTIONS
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
left(1);
right(2);
set(0);
right(1);
set(0);
left(1);
right(2);
set(0);
left(2);
right(3);
set(0);
right(1);
set(0);
left(1);
right(2);
set(0);
left(2);
right(2);
plus(7);
left(2);
right(3);
set(0);
left(8);
set(0);
right(8);
set(0);
left(3);
right(2);
while (tape[ptr]) {
left(2);
right(3);
plus(1);
left(8);
plus(1);
right(7);
minus(1);
left(2);
right(2);
}

left(7);
while (tape[ptr]) {
right(7);
plus(1);
left(7);
minus(1);
}

right(9);
set(0);
left(7);
right(1);
left(1);
right(8);
set(0);
left(10);
set(0);
right(10);
set(0);
left(1);
while (tape[ptr]) {
right(1);
plus(1);
left(10);
plus(1);
right(9);
minus(1);
}

left(9);
while (tape[ptr]) {
right(9);
plus(1);
left(9);
minus(1);
}

right(11);
set(0);
left(2);
right(2);
plus(1);
left(2);
right(3);
set(0);
left(12);
set(0);
right(12);
set(0);
left(3);
right(2);
while (tape[ptr]) {
left(2);
right(3);
plus(1);
left(12);
plus(1);
right(11);
minus(1);
left(2);
right(2);
}

left(11);
while (tape[ptr]) {
right(11);
plus(1);
left(11);
minus(1);
}

right(13);
set(0);
left(4);
right(4);
plus(1);
left(4);
right(5);
set(0);
left(14);
set(0);
right(14);
set(0);
left(5);
right(4);
while (tape[ptr]) {
left(4);
right(5);
plus(1);
left(14);
plus(1);
right(13);
minus(1);
left(4);
right(4);
}

left(13);
while (tape[ptr]) {
right(13);
plus(1);
left(13);
minus(1);
}

right(8);
while (tape[ptr]) {
left(8);
set(0);
right(14);
set(0);
left(5);
right(1);
while (tape[ptr]) {
left(1);
right(5);
plus(1);
left(14);
plus(1);
right(10);
minus(1);
left(1);
right(1);
}

left(10);
while (tape[ptr]) {
right(10);
plus(1);
left(10);
minus(1);
}

right(15);
set(0);
left(15);
set(0);
right(15);
set(0);
left(6);
right(1);
while (tape[ptr]) {
left(1);
right(6);
plus(1);
left(15);
plus(1);
right(10);
minus(1);
left(1);
right(1);
}

left(10);
while (tape[ptr]) {
right(10);
plus(1);
left(10);
minus(1);
}

right(16);
set(0);
left(16);
set(0);
right(16);
set(0);
left(2);
right(1);
while (tape[ptr]) {
left(1);
right(2);
plus(1);
left(16);
plus(1);
right(15);
minus(1);
left(1);
right(1);
}

left(15);
while (tape[ptr]) {
right(15);
plus(1);
left(15);
minus(1);
}

right(17);
set(0);
left(3);
right(3);
plus(48);
left(3);
right(4);
set(0);
left(18);
set(0);
right(18);
set(0);
left(4);
right(3);
while (tape[ptr]) {
left(3);
right(4);
plus(1);
left(18);
plus(1);
right(17);
minus(1);
left(3);
right(3);
}

left(17);
while (tape[ptr]) {
right(17);
plus(1);
left(17);
minus(1);
}

set(0);
right(18);
while (tape[ptr]) {
left(4);
right(2);
plus(1);
left(16);
plus(1);
right(18);
minus(1);
left(4);
right(4);
}

left(18);
while (tape[ptr]) {
right(18);
plus(1);
left(18);
minus(1);
}

set(0);
left(1);
set(0);
right(17);
while (tape[ptr]) {
left(17);
plus(1);
right(1);
plus(1);
right(16);
minus(1);
left(2);
right(2);
}

left(16);
while (tape[ptr]) {
right(16);
plus(1);
left(16);
minus(1);
}

right(18);
set(0);
left(4);
right(2);
set(0);
left(2);
right(3);
set(0);
left(3);
right(3);
set(0);
left(3);
right(1);
set(0);
left(1);
right(1);
set(0);
left(15);
set(0);
right(15);
set(0);
left(16);
while (tape[ptr]) {
right(16);
plus(1);
left(15);
plus(1);
left(1);
minus(1);
}

right(1);
while (tape[ptr]) {
left(1);
plus(1);
right(1);
minus(1);
}

right(15);
printf("%c",(char)(tape[ptr]%4096));
left(1);
right(2);
set(0);
left(2);
right(2);
plus(10);
left(2);
right(2);
printf("%c",(char)(tape[ptr]%4096));
left(2);
right(2);
set(0);
left(2);
right(1);
set(0);
left(1);
right(2);
set(0);
left(16);
set(0);
right(10);
set(0);
left(1);
right(3);
while (tape[ptr]) {
left(3);
right(1);
plus(1);
left(10);
plus(1);
right(12);
minus(1);
left(3);
right(3);
}

left(12);
while (tape[ptr]) {
right(12);
plus(1);
left(12);
minus(1);
}

right(15);
set(0);
left(15);
set(0);
right(15);
set(0);
left(6);
right(3);
while (tape[ptr]) {
left(3);
right(6);
plus(1);
left(15);
plus(1);
right(12);
minus(1);
left(3);
right(3);
}

left(12);
while (tape[ptr]) {
right(12);
plus(1);
left(12);
minus(1);
}

right(16);
set(0);
left(16);
set(0);
right(16);
set(0);
left(2);
while (tape[ptr]) {
right(2);
plus(1);
left(16);
plus(1);
right(14);
minus(1);
}

left(14);
while (tape[ptr]) {
right(14);
plus(1);
left(14);
minus(1);
}

set(0);
right(16);
while (tape[ptr]) {
left(2);
right(1);
plus(1);
left(15);
plus(1);
right(16);
minus(1);
left(2);
right(2);
}

left(16);
while (tape[ptr]) {
right(16);
plus(1);
left(16);
minus(1);
}

set(0);
left(1);
set(0);
right(16);
while (tape[ptr]) {
left(16);
plus(1);
right(1);
plus(1);
right(15);
minus(1);
left(1);
right(1);
}

left(15);
while (tape[ptr]) {
right(15);
plus(1);
left(15);
minus(1);
}

right(15);
set(0);
left(1);
right(2);
set(0);
left(16);
set(0);
right(12);
set(0);
left(13);
while (tape[ptr]) {
right(13);
plus(1);
left(12);
plus(1);
left(1);
minus(1);
}

right(1);
while (tape[ptr]) {
left(1);
plus(1);
right(1);
minus(1);
}

right(15);
set(0);
left(15);
set(0);
right(15);
set(0);
left(10);
right(3);
while (tape[ptr]) {
left(3);
right(10);
plus(1);
left(15);
plus(1);
right(8);
minus(1);
left(3);
right(3);
}

left(8);
while (tape[ptr]) {
right(8);
plus(1);
left(8);
minus(1);
}

right(16);
set(0);
left(2);
right(2);
plus(1);
left(2);
right(3);
set(0);
left(17);
set(0);
right(17);
set(0);
left(3);
right(2);
while (tape[ptr]) {
left(2);
right(3);
plus(1);
left(17);
plus(1);
right(16);
minus(1);
left(2);
right(2);
}

left(16);
while (tape[ptr]) {
right(16);
plus(1);
left(16);
minus(1);
}

set(0);
right(17);
while (tape[ptr]) {
left(3);
right(1);
minus(1);
left(15);
plus(1);
right(17);
minus(1);
left(3);
right(3);
}

left(17);
while (tape[ptr]) {
right(17);
plus(1);
left(17);
minus(1);
}

set(0);
left(1);
set(0);
right(16);
while (tape[ptr]) {
left(16);
plus(1);
right(1);
plus(1);
right(15);
minus(1);
left(1);
right(1);
}

left(15);
while (tape[ptr]) {
right(15);
plus(1);
left(15);
minus(1);
}

right(15);
set(0);
left(1);
right(3);
set(0);
left(17);
set(0);
right(8);
set(0);
left(9);
while (tape[ptr]) {
right(9);
plus(1);
left(8);
plus(1);
left(1);
minus(1);
}

right(1);
while (tape[ptr]) {
left(1);
plus(1);
right(1);
minus(1);
}

right(8);
}

left(3);
right(10);
set(0);
left(15);
set(0);
left(1);
set(0);
right(16);
while (tape[ptr]) {
left(16);
plus(1);
right(1);
plus(1);
right(15);
minus(1);
left(1);
right(1);
}

left(15);
while (tape[ptr]) {
right(15);
plus(1);
left(15);
minus(1);
}

right(11);
set(0);
left(6);
right(3);
set(0);
left(3);
right(4);
set(0);
right(2);
set(0);
left(2);
right(4);
set(0);
left(4);
right(4);
set(0);
left(4);
right(5);
set(0);
right(2);
set(0);
left(2);
right(2);
set(0);
left(2);
right(1);
set(0);
left(6);
set(0);
right(6);
set(0);
left(6);
right(1);
set(0);
left(1);
right(3);
set(0);
left(7);
right(2);
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
left(9);
right(2);
plus(116);
right(1);
plus(101);
right(1);
plus(115);
right(1);
plus(116);
right(1);
plus(105);
right(1);
plus(110);
right(1);
plus(103);
right(1);
plus(1);
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
left(22);
set(0);
right(15);
set(0);
left(10);
right(2);
while (tape[ptr]) {
left(2);
right(10);
plus(1);
left(15);
plus(1);
right(7);
minus(1);
left(2);
right(2);
}

left(7);
while (tape[ptr]) {
right(7);
plus(1);
left(7);
minus(1);
}

right(16);
set(0);
left(11);
right(3);
while (tape[ptr]) {
left(3);
right(11);
plus(1);
left(16);
plus(1);
right(8);
minus(1);
left(3);
right(3);
}

left(8);
while (tape[ptr]) {
right(8);
plus(1);
left(8);
minus(1);
}

right(17);
set(0);
left(8);
while (tape[ptr]) {
right(8);
plus(1);
left(17);
plus(1);
right(9);
minus(1);
}

left(9);
while (tape[ptr]) {
right(9);
plus(1);
left(9);
minus(1);
}

right(18);
set(0);
left(9);
right(1);
while (tape[ptr]) {
left(1);
right(9);
plus(1);
left(18);
plus(1);
right(10);
minus(1);
left(1);
right(1);
}

left(10);
while (tape[ptr]) {
right(10);
plus(1);
left(10);
minus(1);
}

right(19);
set(0);
left(10);
right(2);
while (tape[ptr]) {
left(2);
right(10);
plus(1);
left(19);
plus(1);
right(11);
minus(1);
left(2);
right(2);
}

left(11);
while (tape[ptr]) {
right(11);
plus(1);
left(11);
minus(1);
}

right(20);
set(0);
left(11);
right(3);
while (tape[ptr]) {
left(3);
right(11);
plus(1);
left(20);
plus(1);
right(12);
minus(1);
left(3);
right(3);
}

left(12);
while (tape[ptr]) {
right(12);
plus(1);
left(12);
minus(1);
}

right(21);
set(0);
left(12);
right(4);
while (tape[ptr]) {
left(4);
right(12);
plus(1);
left(21);
plus(1);
right(13);
minus(1);
left(4);
right(4);
}

left(13);
while (tape[ptr]) {
right(13);
plus(1);
left(13);
minus(1);
}

right(22);
set(0);
left(8);
while (tape[ptr]) {
right(8);
plus(1);
left(22);
plus(1);
right(14);
minus(1);
}

left(14);
while (tape[ptr]) {
right(14);
plus(1);
left(14);
minus(1);
}

right(23);
set(0);
left(9);
right(9);
plus(16);
left(9);
right(10);
set(0);
left(24);
set(0);
right(24);
set(0);
left(10);
right(9);
while (tape[ptr]) {
left(9);
right(10);
plus(1);
left(24);
plus(1);
right(23);
minus(1);
left(9);
right(9);
}

left(23);
while (tape[ptr]) {
right(23);
plus(1);
left(23);
minus(1);
}

right(24);
deref();
minus(1);
while (tape[ptr]) {
plus(1);
printf("%c",(char)(tape[ptr]%4096));
right(1);
minus(1);
}

plus(1);
refer();
left(10);
right(11);
set(0);
left(11);
right(11);
plus(10);
left(11);
right(11);
printf("%c",(char)(tape[ptr]%4096));
left(11);
right(11);
set(0);
left(11);
right(11);
set(0);
left(11);
right(10);
set(0);
left(10);
right(9);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
left(12);
right(9);
plus(119);
right(1);
plus(111);
right(1);
plus(119);
right(1);
plus(1);
left(12);
right(13);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
left(30);
set(0);
right(27);
set(0);
left(13);
right(9);
while (tape[ptr]) {
left(9);
right(13);
plus(1);
left(27);
plus(1);
right(23);
minus(1);
left(9);
right(9);
}

left(23);
while (tape[ptr]) {
right(23);
plus(1);
left(23);
minus(1);
}

right(28);
set(0);
left(14);
right(10);
while (tape[ptr]) {
left(10);
right(14);
plus(1);
left(28);
plus(1);
right(24);
minus(1);
left(10);
right(10);
}

left(24);
while (tape[ptr]) {
right(24);
plus(1);
left(24);
minus(1);
}

right(29);
set(0);
left(15);
right(11);
while (tape[ptr]) {
left(11);
right(15);
plus(1);
left(29);
plus(1);
right(25);
minus(1);
left(11);
right(11);
}

left(25);
while (tape[ptr]) {
right(25);
plus(1);
left(25);
minus(1);
}

right(30);
set(0);
left(16);
right(12);
while (tape[ptr]) {
left(12);
right(16);
plus(1);
left(30);
plus(1);
right(26);
minus(1);
left(12);
right(12);
}

left(26);
while (tape[ptr]) {
right(26);
plus(1);
left(26);
minus(1);
}

right(31);
set(0);
left(17);
right(17);
plus(10);
left(17);
right(18);
set(0);
left(32);
set(0);
right(32);
set(0);
left(18);
right(17);
while (tape[ptr]) {
left(17);
right(18);
plus(1);
left(32);
plus(1);
right(31);
minus(1);
left(17);
right(17);
}

left(31);
while (tape[ptr]) {
right(31);
plus(1);
left(31);
minus(1);
}

right(33);
set(0);
left(33);
set(0);
right(33);
set(0);
left(19);
right(18);
while (tape[ptr]) {
left(18);
right(19);
plus(1);
left(33);
plus(1);
right(32);
minus(1);
left(18);
right(18);
}

left(32);
while (tape[ptr]) {
right(32);
plus(1);
left(32);
minus(1);
}

right(34);
set(0);
left(34);
set(0);
right(34);
set(0);
left(20);
right(19);
while (tape[ptr]) {
left(19);
right(20);
plus(1);
left(34);
plus(1);
right(33);
minus(1);
left(19);
right(19);
}

left(33);
while (tape[ptr]) {
right(33);
plus(1);
left(33);
minus(1);
}

right(34);
tape[ptr] = allocate();
left(20);
right(21);
set(0);
left(35);
set(0);
right(35);
set(0);
left(21);
right(20);
while (tape[ptr]) {
left(20);
right(21);
plus(1);
left(35);
plus(1);
right(34);
minus(1);
left(20);
right(20);
}

left(34);
while (tape[ptr]) {
right(34);
plus(1);
left(34);
minus(1);
}

right(36);
set(0);
left(36);
set(0);
right(36);
set(0);
left(22);
right(21);
while (tape[ptr]) {
left(21);
right(22);
plus(1);
left(36);
plus(1);
right(35);
minus(1);
left(21);
right(21);
}

left(35);
while (tape[ptr]) {
right(35);
plus(1);
left(35);
minus(1);
}

set(0);
left(1);
set(0);
right(37);
while (tape[ptr]) {
left(37);
plus(1);
right(1);
plus(1);
right(36);
minus(1);
left(22);
right(22);
}

left(36);
while (tape[ptr]) {
right(36);
plus(1);
left(36);
minus(1);
}

right(33);
set(0);
left(19);
right(21);
set(0);
left(21);
right(22);
set(0);
left(22);
right(19);
set(0);
left(33);
set(0);
right(33);
set(0);
left(34);
while (tape[ptr]) {
right(34);
plus(1);
left(33);
plus(1);
left(1);
minus(1);
}

right(1);
while (tape[ptr]) {
left(1);
plus(1);
right(1);
minus(1);
}

right(34);
set(0);
left(34);
set(0);
right(33);
deref();
set(0);
refer();
left(19);
right(13);
while (tape[ptr]) {
left(13);
right(19);
deref();
plus(1);
refer();
left(33);
plus(1);
right(27);
minus(1);
left(13);
right(13);
}

left(27);
while (tape[ptr]) {
right(27);
plus(1);
left(27);
minus(1);
}

right(33);
deref();
right(1);
set(0);
left(1);
refer();
left(19);
right(14);
while (tape[ptr]) {
left(14);
right(19);
deref();
right(1);
plus(1);
left(1);
refer();
left(33);
plus(1);
right(28);
minus(1);
left(14);
right(14);
}

left(28);
while (tape[ptr]) {
right(28);
plus(1);
left(28);
minus(1);
}

right(33);
deref();
right(2);
set(0);
left(2);
refer();
left(19);
right(15);
while (tape[ptr]) {
left(15);
right(19);
deref();
right(2);
plus(1);
left(2);
refer();
left(33);
plus(1);
right(29);
minus(1);
left(15);
right(15);
}

left(29);
while (tape[ptr]) {
right(29);
plus(1);
left(29);
minus(1);
}

right(33);
deref();
right(3);
set(0);
left(3);
refer();
left(19);
right(16);
while (tape[ptr]) {
left(16);
right(19);
deref();
right(3);
plus(1);
left(3);
refer();
left(33);
plus(1);
right(30);
minus(1);
left(16);
right(16);
}

left(30);
while (tape[ptr]) {
right(30);
plus(1);
left(30);
minus(1);
}

right(35);
set(0);
left(35);
set(0);
right(35);
set(0);
left(21);
right(18);
while (tape[ptr]) {
left(18);
right(21);
plus(1);
left(35);
plus(1);
right(32);
minus(1);
left(18);
right(18);
}

left(32);
while (tape[ptr]) {
right(32);
plus(1);
left(32);
minus(1);
}

right(36);
set(0);
left(22);
right(22);
plus(1);
left(22);
right(23);
set(0);
left(37);
set(0);
right(37);
set(0);
left(23);
right(22);
while (tape[ptr]) {
left(22);
right(23);
plus(1);
left(37);
plus(1);
right(36);
minus(1);
left(22);
right(22);
}

left(36);
while (tape[ptr]) {
right(36);
plus(1);
left(36);
minus(1);
}

set(0);
right(37);
while (tape[ptr]) {
left(23);
right(21);
minus(1);
left(35);
plus(1);
right(37);
minus(1);
left(23);
right(23);
}

left(37);
while (tape[ptr]) {
right(37);
plus(1);
left(37);
minus(1);
}

set(0);
left(1);
set(0);
right(36);
while (tape[ptr]) {
left(36);
plus(1);
right(1);
plus(1);
right(35);
minus(1);
left(21);
right(21);
}

left(35);
while (tape[ptr]) {
right(35);
plus(1);
left(35);
minus(1);
}

right(37);
set(0);
left(23);
right(21);
set(0);
left(21);
right(21);
set(0);
left(35);
set(0);
right(35);
set(0);
left(36);
while (tape[ptr]) {
right(36);
plus(1);
left(35);
plus(1);
left(1);
minus(1);
}

right(1);
while (tape[ptr]) {
left(1);
plus(1);
right(1);
minus(1);
}

right(35);
while (tape[ptr]) {
left(21);
right(22);
set(0);
left(36);
set(0);
right(36);
set(0);
left(22);
right(19);
while (tape[ptr]) {
left(19);
right(22);
plus(1);
left(36);
plus(1);
right(33);
minus(1);
left(19);
right(19);
}

left(33);
while (tape[ptr]) {
right(33);
plus(1);
left(33);
minus(1);
}

right(37);
set(0);
left(37);
set(0);
right(37);
set(0);
left(23);
right(21);
while (tape[ptr]) {
left(21);
right(23);
plus(1);
left(37);
plus(1);
right(35);
minus(1);
left(21);
right(21);
}

left(35);
while (tape[ptr]) {
right(35);
plus(1);
left(35);
minus(1);
}

set(0);
right(37);
while (tape[ptr]) {
left(23);
right(22);
plus(1);
left(36);
plus(1);
right(37);
minus(1);
left(23);
right(23);
}

left(37);
while (tape[ptr]) {
right(37);
plus(1);
left(37);
minus(1);
}

set(0);
left(1);
set(0);
right(37);
while (tape[ptr]) {
left(37);
plus(1);
right(1);
plus(1);
right(36);
minus(1);
left(22);
right(22);
}

left(36);
while (tape[ptr]) {
right(36);
plus(1);
left(36);
minus(1);
}

right(36);
set(0);
left(22);
right(23);
set(0);
left(23);
right(22);
set(0);
left(22);
right(23);
set(0);
left(37);
set(0);
right(37);
set(0);
left(38);
deref();
while (tape[ptr]) {
refer();
right(38);
plus(1);
left(37);
plus(1);
left(1);
deref();
minus(1);
refer();
deref();
}

refer();
right(1);
while (tape[ptr]) {
left(1);
deref();
plus(1);
refer();
right(1);
minus(1);
}

set(0);
right(5);
set(0);
right(32);
while (tape[ptr]) {
left(32);
plus(1);
left(5);
plus(1);
right(37);
minus(1);
left(23);
right(23);
}

left(37);
while (tape[ptr]) {
right(37);
plus(1);
left(37);
minus(1);
}

right(38);
set(0);
left(24);
right(24);
plus(1);
left(38);
set(0);
right(6);
set(0);
left(1);
right(33);
while (tape[ptr]) {
left(33);
right(1);
plus(1);
left(6);
plus(1);
right(38);
minus(1);
left(24);
right(24);
}

left(38);
while (tape[ptr]) {
right(38);
plus(1);
left(38);
minus(1);
}

right(5);
while (tape[ptr]) {
set(0);
right(1);
set(0);
left(1);
}

right(1);
while (tape[ptr]) {
left(1);
right(34);
set(0);
left(39);
set(0);
right(39);
set(0);
left(25);
right(19);
while (tape[ptr]) {
left(19);
right(25);
plus(1);
left(39);
plus(1);
right(33);
minus(1);
left(19);
right(19);
}

left(33);
while (tape[ptr]) {
right(33);
plus(1);
left(33);
minus(1);
}

right(40);
set(0);
left(40);
set(0);
right(40);
set(0);
left(26);
right(21);
while (tape[ptr]) {
left(21);
right(26);
plus(1);
left(40);
plus(1);
right(35);
minus(1);
left(21);
right(21);
}

left(35);
while (tape[ptr]) {
right(35);
plus(1);
left(35);
minus(1);
}

set(0);
right(40);
while (tape[ptr]) {
left(26);
right(25);
plus(1);
left(39);
plus(1);
right(40);
minus(1);
left(26);
right(26);
}

left(40);
while (tape[ptr]) {
right(40);
plus(1);
left(40);
minus(1);
}

set(0);
left(1);
set(0);
right(40);
while (tape[ptr]) {
left(40);
plus(1);
right(1);
plus(1);
right(39);
minus(1);
left(25);
right(25);
}

left(39);
while (tape[ptr]) {
right(39);
plus(1);
left(39);
minus(1);
}

right(39);
set(0);
left(25);
right(26);
set(0);
left(26);
right(25);
set(0);
left(25);
right(26);
set(0);
left(26);
right(26);
plus(1);
left(40);
set(0);
left(1);
deref();
set(0);
refer();
right(41);
while (tape[ptr]) {
left(41);
deref();
plus(1);
refer();
right(1);
plus(1);
right(40);
minus(1);
left(26);
right(26);
}

left(40);
while (tape[ptr]) {
right(40);
plus(1);
left(40);
minus(1);
}

right(6);
set(0);
left(1);
right(1);
}

left(1);
right(36);
set(0);
left(41);
set(0);
right(41);
set(0);
left(27);
right(21);
while (tape[ptr]) {
left(21);
right(27);
plus(1);
left(41);
plus(1);
right(35);
minus(1);
left(21);
right(21);
}

left(35);
while (tape[ptr]) {
right(35);
plus(1);
left(35);
minus(1);
}

right(42);
set(0);
left(28);
right(28);
plus(1);
left(28);
right(29);
set(0);
left(43);
set(0);
right(43);
set(0);
left(29);
right(28);
while (tape[ptr]) {
left(28);
right(29);
plus(1);
left(43);
plus(1);
right(42);
minus(1);
left(28);
right(28);
}

left(42);
while (tape[ptr]) {
right(42);
plus(1);
left(42);
minus(1);
}

set(0);
right(43);
while (tape[ptr]) {
left(29);
right(27);
minus(1);
left(41);
plus(1);
right(43);
minus(1);
left(29);
right(29);
}

left(43);
while (tape[ptr]) {
right(43);
plus(1);
left(43);
minus(1);
}

set(0);
left(1);
set(0);
right(42);
while (tape[ptr]) {
left(42);
plus(1);
right(1);
plus(1);
right(41);
minus(1);
left(27);
right(27);
}

left(41);
while (tape[ptr]) {
right(41);
plus(1);
left(41);
minus(1);
}

right(43);
set(0);
left(29);
right(27);
set(0);
left(41);
set(0);
right(35);
set(0);
left(36);
while (tape[ptr]) {
right(36);
plus(1);
left(35);
plus(1);
left(1);
minus(1);
}

right(1);
while (tape[ptr]) {
left(1);
plus(1);
right(1);
minus(1);
}

right(35);
}

left(35);
set(0);
left(1);
set(0);
right(34);
while (tape[ptr]) {
left(34);
plus(1);
right(1);
plus(1);
right(33);
minus(1);
left(19);
right(19);
}

left(33);
while (tape[ptr]) {
right(33);
plus(1);
left(33);
minus(1);
}

right(27);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
left(16);
right(28);
set(0);
left(28);
right(26);
set(0);
left(26);
right(26);
set(0);
left(26);
right(22);
set(0);
left(22);
right(24);
set(0);
left(24);
right(24);
set(0);
left(24);
right(28);
set(0);
left(28);
right(19);
set(0);
left(19);
right(22);
set(0);
left(22);
right(23);
set(0);
left(23);
right(18);
set(0);
left(18);
right(21);
set(0);
left(21);
right(9);
set(0);
left(23);
set(0);
right(23);
set(0);
left(24);
while (tape[ptr]) {
right(24);
plus(1);
left(23);
plus(1);
left(1);
minus(1);
}

right(1);
while (tape[ptr]) {
left(1);
plus(1);
right(1);
minus(1);
}

right(24);
set(0);
left(24);
set(0);
right(24);
set(0);
left(10);
right(9);
while (tape[ptr]) {
left(9);
right(10);
plus(1);
left(24);
plus(1);
right(23);
minus(1);
left(9);
right(9);
}

left(23);
while (tape[ptr]) {
right(23);
plus(1);
left(23);
minus(1);
}

right(24);
deref();
minus(1);
while (tape[ptr]) {
plus(1);
printf("%c",(char)(tape[ptr]%4096));
right(1);
minus(1);
}

plus(1);
refer();
left(10);
right(11);
set(0);
left(11);
right(11);
plus(10);
left(11);
right(11);
printf("%c",(char)(tape[ptr]%4096));
left(11);
right(11);
set(0);
left(11);
right(10);
set(0);
left(10);
right(11);
set(0);
left(11);
right(10);
set(0);
left(24);
set(0);
right(24);
set(0);
left(10);
right(9);
while (tape[ptr]) {
left(9);
right(10);
plus(1);
left(24);
plus(1);
right(23);
minus(1);
left(9);
right(9);
}

left(23);
while (tape[ptr]) {
right(23);
plus(1);
left(23);
minus(1);
}

right(25);
set(0);
left(11);
right(11);
plus(10);
left(11);
right(12);
set(0);
left(26);
set(0);
right(26);
set(0);
left(12);
right(11);
while (tape[ptr]) {
left(11);
right(12);
plus(1);
left(26);
plus(1);
right(25);
minus(1);
left(11);
right(11);
}

left(25);
while (tape[ptr]) {
right(25);
plus(1);
left(25);
minus(1);
}

right(26);
while (tape[ptr]) {
left(12);
right(13);
set(0);
left(27);
set(0);
right(27);
set(0);
left(13);
right(12);
while (tape[ptr]) {
left(12);
right(13);
plus(1);
left(27);
plus(1);
right(26);
minus(1);
left(12);
right(12);
}

left(26);
while (tape[ptr]) {
right(26);
plus(1);
left(26);
minus(1);
}

right(28);
set(0);
left(14);
right(14);
plus(1);
left(14);
right(15);
set(0);
left(29);
set(0);
right(29);
set(0);
left(15);
right(14);
while (tape[ptr]) {
left(14);
right(15);
plus(1);
left(29);
plus(1);
right(28);
minus(1);
left(14);
right(14);
}

left(28);
while (tape[ptr]) {
right(28);
plus(1);
left(28);
minus(1);
}

set(0);
right(29);
while (tape[ptr]) {
left(15);
right(13);
minus(1);
left(27);
plus(1);
right(29);
minus(1);
left(15);
right(15);
}

left(29);
while (tape[ptr]) {
right(29);
plus(1);
left(29);
minus(1);
}

set(0);
left(1);
set(0);
right(28);
while (tape[ptr]) {
left(28);
plus(1);
right(1);
plus(1);
right(27);
minus(1);
left(13);
right(13);
}

left(27);
while (tape[ptr]) {
right(27);
plus(1);
left(27);
minus(1);
}

right(29);
set(0);
left(15);
right(13);
set(0);
left(27);
set(0);
right(26);
set(0);
left(27);
while (tape[ptr]) {
right(27);
plus(1);
left(26);
plus(1);
left(1);
minus(1);
}

right(1);
while (tape[ptr]) {
left(1);
plus(1);
right(1);
minus(1);
}

right(27);
set(0);
left(27);
set(0);
right(27);
set(0);
left(13);
right(10);
while (tape[ptr]) {
left(10);
right(13);
plus(1);
left(27);
plus(1);
right(24);
minus(1);
left(10);
right(10);
}

left(24);
while (tape[ptr]) {
right(24);
plus(1);
left(24);
minus(1);
}

right(28);
set(0);
left(28);
set(0);
right(28);
set(0);
left(14);
right(12);
while (tape[ptr]) {
left(12);
right(14);
plus(1);
left(28);
plus(1);
right(26);
minus(1);
left(12);
right(12);
}

left(26);
while (tape[ptr]) {
right(26);
plus(1);
left(26);
minus(1);
}

set(0);
right(28);
while (tape[ptr]) {
left(14);
right(13);
plus(1);
left(27);
plus(1);
right(28);
minus(1);
left(14);
right(14);
}

left(28);
while (tape[ptr]) {
right(28);
plus(1);
left(28);
minus(1);
}

set(0);
left(1);
set(0);
right(28);
while (tape[ptr]) {
left(28);
plus(1);
right(1);
plus(1);
right(27);
minus(1);
left(13);
right(13);
}

left(27);
while (tape[ptr]) {
right(27);
plus(1);
left(27);
minus(1);
}

right(27);
set(0);
left(13);
right(14);
set(0);
left(14);
right(13);
set(0);
left(27);
set(0);
right(27);
set(0);
left(28);
while (tape[ptr]) {
right(28);
plus(1);
left(27);
plus(1);
left(1);
minus(1);
}

right(1);
while (tape[ptr]) {
left(1);
plus(1);
right(1);
minus(1);
}

right(28);
set(0);
left(14);
right(13);
deref();
set(0);
refer();
left(13);
right(13);
set(0);
left(13);
right(12);
}

left(12);
right(13);
set(0);
left(18);
right(3);
left(12);
set(0);
left(1);
set(0);
right(28);
while (tape[ptr]) {
left(28);
plus(1);
right(1);
plus(1);
right(27);
minus(1);
left(13);
right(13);
}

left(27);
while (tape[ptr]) {
right(27);
plus(1);
left(27);
minus(1);
}

right(24);
set(0);
left(10);
right(13);
set(0);
left(13);
right(12);
set(0);
left(12);
right(14);
set(0);
left(14);
right(14);
set(0);
left(14);
right(13);
set(0);
left(13);
right(10);
set(0);
left(24);
set(0);
left(1);
set(0);
right(25);
while (tape[ptr]) {
left(25);
plus(1);
right(1);
plus(1);
right(24);
minus(1);
left(10);
right(10);
}

left(24);
while (tape[ptr]) {
right(24);
plus(1);
left(24);
minus(1);
}

right(23);
set(0);
left(9);
right(9);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
left(12);
right(10);
set(0);
left(10);
right(10);
set(0);
left(10);
right(17);
set(0);
left(17);
right(11);
set(0);
left(20);
right(2);
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
right(11);
set(0);
left(20);
right(2);
set(0);
left(2);
right(10);
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
left(17);
right(2);
set(0);
left(2);
right(18);
set(0);
right(1);
set(0);
right(1);
set(0);
right(1);
set(0);
left(12);
right(17);
set(0);
left(26);
right(2);
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
left(15);
set(0);
right(1);
set(0);
right(1);
set(0);
left(1);
right(2);
set(0);
right(1);
set(0);
left(1);
right(2);
set(0);
left(2);
right(3);
set(0);
right(1);
set(0);
}
