#include <stdio.h>
#include "vector.h" // Include your custom vector header

int main() {
    vector myVector; // Declare a custom vector
    initVector(&myVector, 5); // Initialize it with an initial capacity of 5

    // Add elements to the custom vector using pushBack
    pushBack(&myVector, 1);
    pushBack(&myVector, 2);
    pushBack(&myVector, 0);
    pushBack(&myVector, 4);
    pushBack(&myVector, 5);

    // Sort the custom vector
    sortVector(&myVector);

    // Print the sorted vector
    printVector(&myVector);

    pushBack(&myVector,3);
    printVector(&myVector);

    removeAt(&myVector,2);
    printVector(&myVector);

    int num; scanf("%d",&num);
    int index; scanf("%d",&index);
    insertAt(&myVector,num,index);
    printVector(&myVector);
    // Clean up the dynamically allocated memory
    free(myVector.data);

    return 0;
}

