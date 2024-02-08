#include<stdio.h>
#include<math.h>
#define PI 3.141592 //macro

void add(int a[10]){
    int res=0,total;
    printf("enter the number of values to be added: ");
    scanf("%d",&total);
    for(int i=0; i<total; i++){
        scanf("%d",&a[i]);
    }
    for(int i=0;i<total;i++){
        res+=a[i];
    }
    printf("the value of addition is: %d\n",res);
}

int sub(int a, int b){
    printf("enter two values: ");
    scanf("%d %d",&a,&b);
    int result = a-b;
    return result;
    //build a similar function for sub like mult and add
}

void mult(int a[10]){
    int total,res=1;
    printf("enter the number of values to be multiplied: ");
    scanf("%d",&total);
    for(int i=0; i<total; i++){
        scanf("%d",&a[i]);
    }
    for(int i=0;i<total;i++){
        res *= a[i];
    }
    printf("the value of mutliplication is: %d\n",res);
}

void minicalculator(char c){
    printf("choose operation -> a(add), s(sub), m(mult): ");
    int x,y; int arr[10];
    scanf("%c",&c);
    if(c=='m'){
        mult(arr);
    }
    if(c=='a'){
        add(arr);
    }
    if(c=='s'){
        printf("difference is: %d",sub(x,y));
    }
    //similarly build functions for division and trignometric calcs and call them here...
}

int main(){
    /*short int a=2,b=3;*/ char c;
    //printf("%d\n",add(a,b));
    //printf("%d\n",sub(a,b));
    minicalculator(c);
    return 0;
}

/*a hard coded calculator is what we have built. next step is to fill those arrays dynamically and free them.
if user enters more than 10 values for mult and add functions, we get
*stack smashing detected error* --> basically stack-based buffer overflow
*/

//only pass by value being used in function calling
