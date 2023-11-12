#include <stdio.h>
#include <stdlib.h>

int main() {
    printf("Hello, world!\n");

    int arr[7] = {1, 2, 3, 4, 5, 6};

    int **buf = malloc(sizeof(int *) * 6);

    for (int i = 0; i < 6; i++) {
        buf[i] = malloc(sizeof(int) * 7);

        for (int j = 0; j < 7; j++) {
            buf[i][i] = (i + j) * j;
        }
    }
}
