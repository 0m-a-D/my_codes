#include<stdio.h>
void reverse_int(long int n){
    while(n!=0){
        long int quotient = n/10;
        long int remainder = n%10;
        n = quotient;
        printf("%ld",remainder);
    }
}

int main(){
    long int number;
    printf("enter a number to be reversed -> ");
    scanf("%ld",&number);
    reverse_int(number);
    return 0;
}
