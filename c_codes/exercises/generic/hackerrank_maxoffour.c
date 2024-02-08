#include<stdio.h>

int max_of_four(int a,int b,int c,int d){
    int max=0;
    int arr[4]={a,b,c,d};
    for(int i = 0; i < 4; i++){
        if(arr[i]>=max){
            max=arr[i];
        }
    }
    return max;
}

int max_anything(int num, int arr[]){
    int max=0;
    for(int i = 0; i < num; i++){
        if(arr[i]>=max){
            max=arr[i];
        }
    }
    return max;
}
    

int main(){
    int a,b,c,d;
    scanf("%d %d %d %d",&a,&b,&c,&d);
    int ans=max_of_four(a,b,c,d);
    printf("%d\n",ans);
    printf("--------------------TIME TO FIND MAX OF ANYTHING----------------------\n");
    int num;
    printf("enter the number of inputs whose max you need -> ");
    scanf("%d",&num); 
    int arr[num];
    printf("enter those numbers: ");
    for(int i=0;i<num;i++){
        scanf("%d",&arr[i]);
    }
    int max = max_anything(num,arr);
    printf("maximum of these numbers is %d\n",max);
    return 0;
}