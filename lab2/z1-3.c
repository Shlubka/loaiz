#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void multiply_matrices(int n) {
    clock_t start, end; // объявляем переменные для определения времени выполнения
    int i = 0, j = 0, r;
    int **a, **b, **c;

    // Выделение памяти для матриц
    a = (int**)malloc(n * sizeof(int*));
    b = (int**)malloc(n * sizeof(int*));
    c = (int**)malloc(n * sizeof(int*));
    for (i = 0; i < n; i++) {
        a[i] = (int*)malloc(n * sizeof(int));
        b[i] = (int*)malloc(n * sizeof(int));
        c[i] = (int*)malloc(n * sizeof(int));
    }

    srand(time(NULL)); // инициализируем параметры генератора случайных чисел
    for (i = 0; i < n; i++) {
        for (j = 0; j < n; j++) {
            a[i][j] = rand() % 100 + 1; // заполняем массив случайными числами
        }
    }

    srand(time(NULL)); // инициализируем параметры генератора случайных чисел
    for (i = 0; i < n; i++) {
        for (j = 0; j < n; j++) {
            b[i][j] = rand() % 100 + 1; // заполняем массив случайными числами
        }
    }

    start = clock(); // начало измерения времени
    for (i = 0; i < n; i++) {
        for (j = 0; j < n; j++) {
            int elem_c = 0;
            for (r = 0; r < n; r++) {
                elem_c += a[i][r] * b[r][j];
            }
            c[i][j] = elem_c;
        }
    }
    end = clock(); // конец измерения времени

    double time_spent = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Size: %d, Time spent: %f seconds\n", n, time_spent);

    // Освобождение памяти
    for (i = 0; i < n; i++) {
        free(a[i]);
        free(b[i]);
        free(c[i]);
    }
    free(a);
    free(b);
    free(c);
}

int main(void) {
    int sizes[] = {100, 200, 400, 1000, 2000, 4000, 10000};
    int num_sizes = sizeof(sizes) / sizeof(sizes[0]);

    for (int i = 0; i < num_sizes; i++) {
        multiply_matrices(sizes[i]);
    }

    return 0;
}
