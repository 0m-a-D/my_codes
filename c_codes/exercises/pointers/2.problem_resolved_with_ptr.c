#include<stdio.h>
void change_val(int *nb){
    *nb = 24;
}
int main(){
    int nb = 42;
    change_val(&nb);
    printf("%d\n",nb);
    return 0;
}


//example of pass by reference
//
//pass by reference basically means to send pointer as parameter to the called function that points to the location of the original variable thereby allowing us to make changes to the variable from the called function.
