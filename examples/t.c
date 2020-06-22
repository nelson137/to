#include <stdio.h>

int main(int argc, char *argv[]) {
    puts("hello from c");
    for (int i=0; i<argc; ++i)
        printf("  %d : %s\n", i, argv[i]);
    return 0;
}
