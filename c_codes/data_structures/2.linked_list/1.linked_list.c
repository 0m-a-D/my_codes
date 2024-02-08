#include<stdio.h>

struct node
{
    int value;
    struct node* next; 
};

typedef struct node node_t; 
//basically giving an alias to an existing data type. we can give alias to an alias (it's not necessary, but possible)

void printlist(node_t *head){
    node_t *temporary = head;
    while (temporary!=NULL)
    {
        printf("%d -> ",temporary->value);
        temporary = temporary -> next;
    }
    printf("\n");
}

int main(void){
    node_t n1,n2,n3,n4,n5;
    node_t *head;
    n1.value = 1;
    n2.value = 2;
    n3.value = 3;
    n4.value = n2.value<<1; //left bit shifting by 2 bits...gives 8...change the values to understand better
    
    //link them up
    head = &n3;
    n3.next = &n5;
    n2.next = &n1;
    n1.next = &n4; //so we know when to stop
    n4.next = NULL; //n4 is between n2 and n1...n3->n2->n4->n1  |  n3->n5->n2->n1->n4
    n5.value = 5;
    n5.next = &n2;

    printlist(head);
    return 0;
}