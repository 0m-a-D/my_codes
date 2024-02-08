#include<stdio.h>
#include<stdarg.h>

int println(const char *__restrict __format_, ...){
    va_list args;
    va_start(args, __format_);
    int result = vprintf(__format_, args);
    va_end(args);
    printf("\n");
    return result;
}

int main(){
    println("my own println function");
    //int result  = printf("hello\n");
    //printf("%d\n",result);
    //printf("my own println function\n");
    return 0;
}