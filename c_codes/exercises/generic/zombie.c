#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>
int main(int argc, char *argv[]){
    while(true){
        if(fork() == 0){
            printf("I am a zombie process, my pid is %d\n", getpid());
            exit(0);
        }
    }
}