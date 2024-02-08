#include<stdio.h>
#include<error.h>
int sum(int a, int b){
    return a+b;
}

void iseven(int n){
    if(n%2==0){
        printf("number is even\n");
    }
    else{
        perror("error");
    }
}

int main(){
    int result = sum(3,4);
    printf("%d\n",result);
    int n; scanf("%d",&n);
    iseven(n);
}
