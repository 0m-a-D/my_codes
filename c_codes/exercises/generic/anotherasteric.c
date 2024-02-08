#include<stdio.h>
int main(){
    int num_rows=0;
    printf("enter the number of rows: ");
    scanf("%d",&num_rows);
    for(int i = 1; i<=num_rows;i++){
        for(int j=0; j<i;j++){
            printf("*");
        }
        printf("\n");
    }
    return 0;
}
/*
 this would print the pattern:
 *
 **
 ***
 and so on.....
 */