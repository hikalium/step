#include <stdio.h>
#include <unistd.h>
int main() {
  const char *pass = "3141";
  char s[5];
  puts("Input your passcode (4 digits)");
  fgets(s, sizeof(s), stdin);
  for (int i = 0; i < 4; i++) {
    sleep(1);
    if (s[i] != pass[i]) {
      puts("Error!");
      return 0;
    }
  }
  puts("Welcome!");
  return 0;
}
