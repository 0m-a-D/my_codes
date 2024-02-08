#include<stdio.h>
int main(){
    printf("enter the character whose ascii you want -> ");
    char *alphabet; 
    scanf("%c",alphabet);  
    printf("%d",*alphabet);
    return 0;
}
