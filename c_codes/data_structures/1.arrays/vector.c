#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int* data;
    size_t size;
    size_t capacity;
} vector;

void initVector(vector* vec, size_t initialCapacity) {
    vec->data = (int*)malloc(initialCapacity * sizeof(int));
    vec->size = 0;
    vec->capacity = initialCapacity;
}

void pushBack(vector* vec, int value) {
    if (vec->size == vec->capacity) {
        // Resize the vector if it's full
        vec->capacity *= 2;
        vec->data = (int*)realloc(vec->data, vec->capacity * sizeof(int));
    }
    vec->data[vec->size++] = value;
}

void insertAt(vector* vec, int value, size_t index) {
    if (index >= vec->size) {
        // If the index is greater than or equal to the current size, simply push the value to the end.
        pushBack(vec, value);
    } else {
        if (vec->size == vec->capacity) {
            // Resize the vector if it's full
            vec->capacity *= 2;
            vec->data = (int*)realloc(vec->data, vec->capacity * sizeof(int));
        }
        for (size_t i = vec->size; i > index; i--) {
            vec->data[i] = vec->data[i - 1];
        }
        vec->data[index] = value;
        vec->size++;
    }
}

void removeAt(vector* vec, size_t index) {
    if (index < vec->size) {
        for (size_t i = index; i < vec->size - 1; i++) {
            vec->data[i] = vec->data[i + 1];
        }
        vec->size--;
    }
}

void sortVector(vector* vec) {
    // Simple bubble sort implementation for demonstration purposes
    for (size_t i = 0; i < vec->size - 1; i++) {
        for (size_t j = 0; j < vec->size - i - 1; j++) {
            if (vec->data[j] > vec->data[j + 1]) {
                int temp = vec->data[j];
                vec->data[j] = vec->data[j + 1];
                vec->data[j + 1] = temp;
            }
        }
    }
}

void printVector(const vector* vec) {
    for (size_t i = 0; i < vec->size; i++) {
        printf("%d ", vec->data[i]);
    }
    printf("\n");
}