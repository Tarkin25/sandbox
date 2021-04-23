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

Node **newList() {
    return calloc(1, sizeof(Node*));
}

void addItem(Node **list, char *item) {
    Node *current = *list;

    while(current != NULL && current->next != NULL && compareIgnoreCase(current->next->item, item) < 1) {
        current = current->next;
    }

    if(current == *list) {
        current = malloc(sizeof(Node*));
        current->item = item;
        current->next = *list;
        *list = current;
    } else {
        Node *new = malloc(sizeof(Node));
        new->item = item;
        new->next = current->next;
        current->next = new;
    }
}

void printItems(Node **list) {
    Node *current = *list;

    while(current != NULL) {
        printf("%s\n", current->item);
        current = current->next;
    }
}

int main() {

    Node **list = newList();

    addItem(list, "Beer");
    addItem(list, "Sausage");
    addItem(list, "Chips");
    addItem(list, "Cider");
    addItem(list, "Apples");
    addItem(list, "Tonic");
    addItem(list, "zafron");
    addItem(list, "apple juice");

    printItems(list);

    return 0;
}
