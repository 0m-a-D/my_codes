#include<stdio.h>
void fizzbuzz(int num){
    for(int i=1; i<num+1;i++){
        if(i%3==0&&i%5==0){
            printf("fizzbuzz\n");
        }
        else if(i%3==0){
            printf("fizz\n");
        }
        else if(i%5==0){
            printf("buzz\n");
        }
        else{
            printf("%d\n",i);
        }
    }
}
int main(){
    int num;
    printf("enter the limit till which fizzbuzz should be executed -> ");
    scanf("%d",&num);
    fizzbuzz(num);
    printf("finished fizzbuzz...exiting!!\n");
    return 0;
}