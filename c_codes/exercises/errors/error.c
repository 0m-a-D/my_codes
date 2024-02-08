#include<stdio.h>
#include<errno.h>
int main(){
    FILE* fp;
    fp=fopen("hello.txt","r");
    printf("value of errno is -> %d\n",errno);
    return 0;
}
