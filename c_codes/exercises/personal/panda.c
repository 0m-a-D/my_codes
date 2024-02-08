#include<stdio.h>
#include<unistd.h>
int main(){
    int num; 
    printf("on the scale of 1 to 10, how much do you love me -> ");
    scanf("%d",&num);
    if(num>10 || num<1){
        printf("naughtly panda, I'm serious!\n");
    }
    else{
        printf("noish! guess how much do I do\n");
        sleep(2);
        while(1){
            printf("I love you more!\n");
        }
    }
    return 0;
}