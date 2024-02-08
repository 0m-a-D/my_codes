#include<stdio.h>
void change_arr(int i,int arr[i]){
    arr[1]=69;
    for(int j=0;j<i;j++){
        printf("%d ",arr[j]);
    }
}
void change_num(int *num){
    int d = *num+1;
    printf("%d",d);
}
int main(){
    int n;
    printf("enter the number of the elemets of the array -> ");
    scanf("%d",&n); int arr[n];
    for(int i=0;i<n;i++){
        scanf("%d",&arr[i]);
    }
    change_arr(n,arr);
    printf("\n");
    //notice how array is being passed by value yet the called function changes a value of the array and the result in being 
    //reflected in the caller function ,i.e main().


    //but what about normal variables. How would we change those?
    int random; printf("enter a random variable -> ");
    scanf("%d",&random);
    change_num(&random); printf("\n");
    //what we did here is pass by reference.
    //
    int new_arr[5];
    for(int i=0;i<5;i++){
        printf("%d ",new_arr[i]); //all values shall be 0
    }
    printf("\n");
    return 0;
}