#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main(){
    //keep getting input from input stream
    int c;
    while((c=getchar())!=EOF){
        putchar(c);
    }
    printf("\n");
    return 0;
}
