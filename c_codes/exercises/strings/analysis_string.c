#include <stdio.h>
#include <string.h>
#include <ctype.h>

void reverseStringPreservingFormat(char* input) {
    int length = strlen(input);
    int start = 0;
    int end = length - 1;
    
    while (start < end) {
        // Skip non-alphabetic characters
        if (!isalpha(input[start])) {
            start++;
        } else if (!isalpha(input[end])) {
            end--;
        } else {
            // Swap alphabetic characters
            char temp = input[start];
            input[start] = input[end];
            input[end] = temp;
            start++;
            end--;
        }
    }
}

int main() {
    char input[100];
    
    printf("Enter a string: ");
    fgets(input, sizeof(input), stdin);
    input[strcspn(input, "\n")] = '\0'; // Remove newline character
    
    reverseStringPreservingFormat(input);
    
    printf("Output: %s\n", input);
    
    return 0;
}
