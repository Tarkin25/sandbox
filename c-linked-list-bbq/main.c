#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char lowerCase(char c) {
    return c > 64 && c < 91 ? c + 32 : c;
}

int compareIgnoreCase(const char *a, const char *b) {
    int i = 0;
    char lowerA;
    char lowerB;

    while(a[i] != '\0' && b[i] != '\0') {
        lowerA = lowerCase(a[i]);
        lowerB = lowerCase(b[i]);

        if(lowerA < lowerB) return -1;
        else if (lowerA > lowerB) return 1;
        else i++;
    }

    i++;

    if(a[i] == '\0' && b[i] != '\0') return -1;
    else if (a[i] != '\0' && b[i] == '\0') return 1;
    else return 0;
}

typedef struct SNode {
    char *item;
    struct SNode *next;
} Node;

Node *newList() {
    return calloc(1, sizeof(Node));
}

void addItem(Node *head, char *item) {
    while(head->next != NULL && compareIgnoreCase(head->next->item, item) < 1) {
        head = head->next;
    }

    Node* new = malloc(sizeof(Node));
    new->item = item;
    new->next = head->next;
    head->next = new;
}

void printItems(Node *head) {
    Node *current = head->next;

    while(current != NULL) {
        printf("%s\n", current->item);
        current = current->next;
    }
}

int main() {

    Node *head = newList();

    addItem(head, "Beer");
    addItem(head, "Sausage");
    addItem(head, "Chips");
    addItem(head, "Cider");
    addItem(head, "Apples");
    addItem(head, "Tonic");
    addItem(head, "zafron");
    addItem(head, "apple juice");

    printItems(head);

    return 0;
}
