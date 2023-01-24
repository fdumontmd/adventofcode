#include <stdio.h>
// because sometimes you really need goto; showed that the 4th register value looped
int main() {
  unsigned long r0, r1, r2, r3, r4, r5;
  r0 = r1 = r2 = r3 = r4 = r5 = 0;

//seti 0 8 4
  r4 = 0;
l6:
//bori 4 65536 3
  r3 = r4 | 65536;
//seti 14464005 5 4
  r4 = 14464005;
l8:
//bani 3 255 2
  r2 = r3 & 255;
//addr 4 2 4
  r4 += r2;
//bani 4 16777215 4
  r4 &= 16777215;
//muli 4 65899 4
  r4 *= 65899;
//bani 4 16777215 4
  r4 &= 16777215;
//gtir 256 3 2
  r2 = 256 > r3;
//addr 2 5 5
//addi 5 1 5
//seti 27 7 5
  if (r2) goto l28;
//seti 0 3 2
  r2 = 0;
l18:
//addi 2 1 1
  r1 = r2 + 1;
//muli 1 256 1
  r1 *= 256;
//gtrr 1 3 1
  r1 = r1 > r3;
//addr 1 5 5
//addi 5 1 5
//seti 25 2 5
  if (r1) goto l26;
//addi 2 1 2
  r2 += 1;
//seti 17 9 5
  goto l18;
l26:
//setr 2 2 3
  r3 = r2;
//seti 7 3 5
  goto l8;
l28:
//eqrr 4 0 2
  printf("%lu\n", r4);
  r2 = r4 == r0;
//addr 2 5 5
  if (r2) return 0;
//seti 5 9 5
  goto l6;


  return 0;
}
