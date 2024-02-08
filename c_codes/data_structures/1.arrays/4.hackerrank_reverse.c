#include<stdio.h>

//one way of reversing...swapping with indexing
void swapping(int num,int arr[]){
    int i;
    for(i=0;i<num;i++){
        scanf("%d",&arr[i]);
    }
    for (i=0;i<num/2;i++){
        int temp = arr[i];
        arr[i]=arr[num-i-1];
        arr[num-i-1]=temp;
    }
    for(i=0;i<num;i++){
        printf("%d ",arr[i]);
    }
    printf("\n");
}
int main(){
    int num;
    scanf("%d",&num);
    int arr[num];
    swapping(num,arr);
    return 0;
}