#include <stdio.h>
#define eM(x) *p*x;
#define eL(x) *(p+x)+=
#define eK(x) *p*x;
#define eJ(x) *(p-x)+=
#define eI(x) *p=x;
#define eH(x) }
#define eG(x) while(*p){
#define eF(x) p+=x;
#define eE(x) p-=x;
#define eD(x) putchar(*p);
#define eC(x) *p-=x;
#define eB(x) c=getchar();if(c>=0)*p=c;
#define eA(x) *p+=x;
char buf[0x10100];
int main(){
char *p=buf+127;
int c;
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(7)
eF(1)
eI(0)
eE(8)
eI(0)
eF(8)
eI(0)
eE(1)
eL(1)
eM(1)
eJ(7)
eK(1)
eI(0)
eE(7)
eL(7)
eM(1)
eI(0)
eF(8)
eD(0)
eI(0)
eE(1)
eI(0)
eE(8)
eI(0)
eF(1)
eI(0)
eE(1)
eI(0)
eF(8)
eJ(8)
eK(1)
eJ(7)
eK(1)
eI(0)
eE(7)
eL(7)
eM(1)
eI(0)
eF(7)
eI(0)
eE(8)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eF(1)
eI(0)
eE(7)
return 0;}
