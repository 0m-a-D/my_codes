#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main() 
{

    /* Enter your code here. Read input from STDIN. Print output to STDOUT */ 
    char s; char str[20]; char sen[50];
    scanf("%c\n",&s);
    scanf("%[^\n]%*c", str); 
    scanf("%[^\n]%*c", sen);
    
    printf("%c\n",s);
    printf("%s\n", str);
    printf("%s\n", sen);
    return 0;
}