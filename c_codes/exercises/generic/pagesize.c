#include<stdio.h>
#include<unistd.h>
int main(){
    int x = sysconf(_SC_PAGESIZE);
    printf("%d\n",x);
    return 0;
}