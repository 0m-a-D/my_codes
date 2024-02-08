#include<stdio.h>
#include<unistd.h>
int main(){
    int x = sysconf(_SC_PAGE_SIZE); int y = sysconf(_SC_PHYS_PAGES);
    int z = sysconf(_SC_AVPHYS_PAGES);
    printf("%d\n",x);
    printf("%d\n",y);
    printf("%d\n",z);
    printf("total memory -> %ld\n",x*y);
    printf("memory available -> %ld\n",z*y);
    return 0;
}