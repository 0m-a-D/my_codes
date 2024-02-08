#include<stdio.h>

int absolute(int x){
    if(x<0){
        x=-x;
    }
    else{
        x=x;
    }
    return x;
}
void update(int *a,int *b) {
    //Complete this function   
    int tmp = *a;
    *a = *a+*b;
    int diff = tmp - *b;

    *b=absolute(diff);
}
int main(){
    int a=3,b=9;
    int *p=&a,*q=&b;
    update(p,q);
    printf("%d\n%d\n",a,b);
    return 0;
}
