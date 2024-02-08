#include<stdio.h>
int main(){
    int c,nl=0;
    while((c=getchar()) != EOF){
        if(c == '\n'){
            ++nl;
        }
    }
    printf("%d",nl);
    return 0;
}

// 'character' basically refers to the ASCII value of the character. to assign a character, we use "character".