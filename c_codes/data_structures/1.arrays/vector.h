#ifndef VECTOR_H
#define VECTOR_H

#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int* data;
    size_t size;
    size_t capacity;
} vector;

void initVector(vector* vec, size_t initialCapacity);
void pushBack(vector* vec, int value);
void sortVector(vector* vec);
void printVector(const vector* vec);
void insertAt(vector* vec, int value, size_t index);
void removeAt(vector* vec, size_t index);
#endif