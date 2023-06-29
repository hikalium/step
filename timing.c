#include <stdio.h>
int main() {
  const char *pass = "3141";
  char s[4];
  puts("Input your passcode (4 digits)");
  fgets(s, 4, stdin);
  for (int i = 0; i < 4; i++) {
    if (s[i] != pass[i]) {
      puts("Error!");
      return 1;
    }
  }
  puts("Welcome!");
  return 0;
}
