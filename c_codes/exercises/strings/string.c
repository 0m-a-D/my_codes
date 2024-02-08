#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main(){
    char greet[] = "hello";
    printf("%s\n",greet);

    //to print a particular character from a string 
    printf("%c\n",greet[4]);
    printf("I am %s years old\n", greet);
    printf("length of the string is %ld\n",strlen(greet));
    printf("length of the string is %ld\n",sizeof(greet)); //gives 6 because of null character '\0'
    printf("length of the string is %ld\n",sizeof(greet)/sizeof(char) - 1);

    char str[2];
    scanf("%s",str);
    
    printf("%s\n",str);

    return 0;
}