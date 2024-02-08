#include <stdio.h>
int main()
{
    // code for matrix multiplication
    int row1, col1, row2, col2;

    printf("enter rows and columns for first matrix:");
    scanf("%d %d", &row1, &col1);

    printf("enter rows and columns for second matrix:");
    scanf("%d %d", &row2, &col2);

    if (col1 == row2)
    {
        printf("enter elements of first matrix: ");
        int mat1[row1][col1];
        for (int i = 0; i < row1; i++)
        {
            for (int j = 0; j < col1; j++)
            {
                scanf("%d", &mat1[i][j]);
            }
        }

        printf("enter elements of second matrix: ");
        int mat2[row2][col2];
        for (int i = 0; i < row2; i++)
        {
            for (int j = 0; j < col2; j++)
            {
                scanf("%d", &mat2[i][j]);
            }
        }

        int result[row1][col2];
        for (int i = 0; i < row1; i++)
        {
            for (int j = 0; j < col2; j++)
            {
                result[i][j] = 0;
                for (int k = 0; k < col1; k++)
                {
                    result[i][j] += mat1[i][k] * mat2[k][j];
                }
            }
        }

        // print result of matrix multiplication...
        printf("Result of matrix multiplication: \n");
        for (int i = 0; i < row1; i++)
        {
            for (int j = 0; j < col2; j++)
            {
                printf("%d ",result[i][j]);
            }
            printf("\n");
        }
    }
    else
    {
        printf("matrix multiplication is not possible!\n");
    }
    return 0;
}