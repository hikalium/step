#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
uint8_t data[128 * 1024 * 1024];
int main() {
  srand(0);
  for(int i = 0; i < sizeof(data); i++)
     data[i] = rand(); 
  puts("init done!");
  uint64_t sum = 0;
  for(int i = 0; i < sizeof(data) / 1024; i++)
    for(int k = 0; k < 1024; k++)
     sum += data[i * 1024 + k];
  printf("sum = %lu\n", sum);
  return 0;
}
