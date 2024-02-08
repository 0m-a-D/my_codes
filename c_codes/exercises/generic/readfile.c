#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_WORD_LENGTH 50
#define MAX_WORDS 1000

int main() {
    char filename[] = "words.txt";
    char words[MAX_WORDS][MAX_WORD_LENGTH];
    char search_word[MAX_WORD_LENGTH];
    int num_words = 0;
    int i;

    // Open file
    FILE* fp = fopen(filename, "r");
    if (fp == NULL) {
        printf("Error opening file.\n");
        return 1;
    }

    // Read words from file
    while (fscanf(fp, "%s", words[num_words]) != EOF) {
        num_words++;
    }

    // Close file
    fclose(fp);

    // Prompt user for word to search
    printf("Enter a word to search: ");
    scanf("%s", search_word);

    // Search for word in list
    for (i = 0; i < num_words; i++) {
        if (strcmp(words[i], search_word) == 0) {
            printf("Word found.\n");
            return 0;
        }
    }

    // Word not found
    printf("Word not found.\n");
    return 0;
}
