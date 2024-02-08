//delete an element from middle of the array and give the updated array
#include<stdio.h>
#include<stdlib.h>

void deleting(int del, int arr[], int num){
    for(int i=del-1;i<num-1;i++){
        arr[i]=arr[i+1];
    }
    
    printf("the resultant array after deletion is -> ");
    for(int i=0;i<num-1;i++){
        printf("%d ",arr[i]);
    }
}

int main(){
    int num,del;
    printf("enter the number of elements -> ");
    scanf("%d",&num); 
    
    int arr[num];
    
    printf("enter the elements -> ");
    for(int i = 0; i < num; i++){
        scanf("%d",&arr[i]);
    }
    
    printf("your array is -> ");
    for(int i = 0; i < num;  i++){
        printf("%d ",arr[i]);
    }
    
    printf("\n");
    
    printf("which element would you like to delete(ex..1 for first element and so on) -> ");
    scanf("%d",&del);
    
    deleting(del,arr,num);
    
    printf("\n");
    printf("size of new array is %ld\n",sizeof(arr)/sizeof(int));
    return 0;
}

// WHY YOU COULDN'T USE THE MACRO "SIZE" TO DO WHAT "NUM" DID WHEN PASSED BY VALUE
//because macros are resolved at compile time. this means from the beginning itself size was 0 as 
//array was having no element hence size was 0.
