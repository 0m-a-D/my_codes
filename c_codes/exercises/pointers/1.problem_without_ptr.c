#include<stdio.h>
void change_val(int nb);
int main(){
    int nb = 42;
    change_val(nb);
    printf("%d\n",nb);
    return 0;
}
void change_val(int nb){
    nb = 24;
}

//what do you think the output is going to be?
//   -> 42
//
//reason: main() and change_val() are two different frames in the stack. nb in change_val is not the same as nb in main function. thus we assigned the nb as 24 of the change_val() func but nothing really changes to main() frame.
//can be solved using pointers...
//
//
//
//example of pass by value