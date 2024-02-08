#include<stdio.h>
int main(){
    int a=0,b=0,c=0;
    a++;                //post increment
    printf("%d\n",a);
    int d = b++;        //assigning with post increment
    printf("%d\n",d);   //what is the value of d?
    ++c;                //pre increment
    printf("%d\n",c);
    printf("%d\n",b); //what is the value of b?
    int z;
    for(z=0;z<5;++z){
        printf(".");
    }
    printf("%d\n",z);

    printf("--------------------------------\n");
    for(int i = 0;i<5;i++){
        printf("%d\n",i);
    }
    return 0;
}
//if line 7 and 10 are different values, why is that the case? why not the same?