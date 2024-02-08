#include<stdio.h>
int main(){
    int rem = 5%2;
    float ano_rem = 4%2;
    printf("%d\n",rem);
    printf("%f\n",ano_rem);
    printf("%.1d\n",5/2);
    printf("%.2f\n",(float)5/(float)4);
    int a = 5;
    a += 1;
    printf("a is %d\n",a);
    int b = 0;
    for (int i = 0; i < a; i++)
    {
        b+=1;
    }
    printf("b is %d\n",b);

    int c = 0;
    LOOP:
    c+=1;
    if(c<10){
        goto LOOP;
    }
    else{
        goto HERE;
    }
    HERE:
    printf("%d\n",c);
    return 0;
}
