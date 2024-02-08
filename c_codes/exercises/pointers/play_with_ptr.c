#include<stdio.h>
#include<time.h>
void swap_by_ptr(int *first,int *second){
    int temp = *second;
    *second = *first;
    *first = temp;
    printf("a=%d\tb=%d\n",*first,*second);
}
void swap_by_val(int a, int b){
    int temp = b;
    b = a;
    a = temp;
    printf("a=%d\tb=%d\n",a,b);
}
int main(){
    int a=1,b=3;
    swap_by_ptr(&a,&b); //works...pass by reference
    swap_by_val(a,b);   //failed...made a copy and made change to that copy

    int *n1=&a;
    int *n2=&b; int c=10; int d=0; int e = sizeof(d);
    printf("%d\n",*n1);
    printf("%d\n",*n2);
    printf("n1 points to (a's) address -> %p\n",n1);
    printf("n2 points to (b's) address ->%p\n",n2);
    printf("a address -> %p\n",&a);
    printf("b address -> %p\n",&b);
    printf("c address -> %p\n",&c);
    printf("d address -> %p\n",&d);
    printf("%d\n",e);
    return 0;
}
