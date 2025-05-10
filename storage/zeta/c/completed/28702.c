#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int getNextInt(char **s1) {
    for (int i = 0; i < 3; i++) {
        if (s1[i][0] == 'F' || s1[i][0] == 'B') {
            continue;
        } else {
            return atoi(s1[i]) + (3 - i);
        }
    }

    return 0;
}

void getNext(char *s2, char **s1) {
    int ni = getNextInt(s1);
    if (ni % 3 == 0 && ni % 5 == 0) {
        strcpy(s2, "FizzBuzz");
    } else if (ni % 3 == 0) {
        strcpy(s2, "Fizz");
    } else if (ni % 5 == 0) {
        strcpy(s2, "Buzz");
    } else {
        sprintf(s2, "%d", ni);
    }
}

int main() {
    char **s1 = malloc(sizeof(char *) * 3);
    if (s1 == NULL) {
        exit(1);
    }
    for (int i = 0; i < 3; i++) {
        s1[i] = malloc(sizeof(char) * 10);
        if (s1[i] == NULL) {
            exit(1);
        }
    }

    for (int i = 0; i < 3; i++) {
        scanf("%s", s1[i]);
    }

    char *s2 = malloc(sizeof(char) * 10);
    if (s2 == NULL) {
        exit(1);
    }
    getNext(s2, s1);
    printf("%s\n", s2);

    free(s2);

    for (int i = 0; i < 3; i++) {
        free(s1[i]);
    }
    free(s1);
    return 0;
}