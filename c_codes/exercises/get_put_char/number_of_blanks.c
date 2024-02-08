#include<stdio.h>
int main(){
    int c, no_of_blanks=0;
    while((c=getchar())!=EOF){
        /*if(c==' '){
            ++no_of_blanks;
        }
        */
        
        //OR

        if(c==32){
            ++no_of_blanks;
        }
    }
    printf("%d",no_of_blanks);
    return 0;
}
//ASCII value of a space character is 32.