#include<stdio.h>
int main(){
    int num_rows=0;
    printf("enter the number of rows: ");
    scanf("%d",&num_rows);
    for(int i=num_rows; i>0; i--){
        for(int j = i; j>0;j--){
            printf("*");
        }
        printf("\n");
    }
    return 0;
}
/*
***
**
*
and so on.....
*/