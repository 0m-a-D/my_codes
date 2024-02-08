#include <stdio.h>

void sort(int num, int arr[])
{
    for (int i = 0; i < num - 1; ++i)
    {
        for (int j = 0; j < num - i - 1; ++j)
        {
            if (arr[j] > arr[j + 1])
            {
                int temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    // understand the logic of this loop and sorting (bubble)!!

    for (int i = 0; i < num; i++)
    {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

int main()
{
    int num;
    scanf("%d", &num);
    int arr[num];
    int i;
    // take array as input
    for (i = 0; i < num; i++)
    {
        scanf("%d", &arr[i]);
    }
    // sort the array
    // sort(num,arr);
    for (int i = 0; i < 2; ++i)
    {
        printf("%d ", i);
    }
    return 0;
}