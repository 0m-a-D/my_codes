#include <stdio.h>
#include <string.h>

int main() {
    char firststr[] = "youarehereforhero";
    char pattern[] = "hero";
    
    int i, j, match;
    for (i = 0; i <= strlen(firststr) - strlen(pattern); i++) {
        match = 1;
        for (j = 0; j < strlen(pattern); j++) {
            if (firststr[i+j] != pattern[j]) {
                match = 0;
                break;
            }
        }
        if (match) {
            printf("Pattern found at index %d\n", i);
            break;
        }
    }
    if (!match) {
        printf("Pattern not found\n");
    }
    return 0;
}