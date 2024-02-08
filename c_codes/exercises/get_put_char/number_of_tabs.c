#include<stdio.h>
int main(){
    int c, no_of_tabs=0;
    while((c=getchar())!=EOF){
        if(c==9){             // '    ' holds TAB
            ++no_of_tabs;
        }
    }
    printf("%d\n",no_of_tabs);
    return 0;
}