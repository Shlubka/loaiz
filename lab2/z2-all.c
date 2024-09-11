#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void shell(int *items, int count);
void qs(int *items, int left, int right);

void fill_random(int *items, int count);
void fill_ascending(int *items, int count);
void fill_descending(int *items, int count);
void fill_half_ascending_half_descending(int *items, int count);
void print_array(int *items, int count);

int cmpfunc(const void *a, const void *b);

int main() {
    int counts[] = {100, 1000, 10000};
    for (int i = 0; i < 3; i++) {
        int count = counts[i];
        printf("count == %d\n", count);
        int *items = (int *)malloc(count * sizeof(int));

        fill_random(items, count);
        clock_t start = clock();
        shell(items, count);
        clock_t end = clock();
        printf("Shell sort (random): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_random(items, count);
        start = clock();
        qs(items, 0, count - 1);
        end = clock();
        printf("Quick sort (random): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_random(items, count);
        start = clock();
        qsort(items, count, sizeof(int), cmpfunc);
        end = clock();
        printf("qsort (random): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_ascending(items, count);
        start = clock();
        shell(items, count);
        end = clock();
        printf("Shell sort (ascending): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_ascending(items, count);
        start = clock();
        qs(items, 0, count - 1);
        end = clock();
        printf("Quick sort (ascending): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_ascending(items, count);
        start = clock();
        qsort(items, count, sizeof(int), cmpfunc);
        end = clock();
        printf("qsort (ascending): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_descending(items, count);
        start = clock();
        shell(items, count);
        end = clock();
        printf("Shell sort (descending): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_descending(items, count);
        start = clock();
        qs(items, 0, count - 1);
        end = clock();
        printf("Quick sort (descending): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_descending(items, count);
        start = clock();
        qsort(items, count, sizeof(int), cmpfunc);
        end = clock();
        printf("qsort (descending): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_half_ascending_half_descending(items, count);
        start = clock();
        shell(items, count);
        end = clock();
        printf("Shell sort (half ascending, half descending): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_half_ascending_half_descending(items, count);
        start = clock();
        qs(items, 0, count - 1);
        end = clock();
        printf("Quick sort (half ascending, half descending): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        fill_half_ascending_half_descending(items, count);
        start = clock();
        qsort(items, count, sizeof(int), cmpfunc);
        end = clock();
        printf("qsort (half ascending, half descending): %f seconds\n", (double)(end - start) / CLOCKS_PER_SEC);

        free(items);
        printf("\n");
    }
    return 0;
}

void fill_random(int *items, int count) {
    for (int i = 0; i < count; i++) {
        items[i] = rand() % count;
    }
}

void fill_ascending(int *items, int count) {
    for (int i = 0; i < count; i++) {
        items[i] = i;
    }
}

void fill_descending(int *items, int count) {
    for (int i = 0; i < count; i++) {
        items[i] = count - i - 1;
    }
}

void fill_half_ascending_half_descending(int *items, int count) {
    for (int i = 0; i < count / 2; i++) {
        items[i] = i;
    }
    for (int i = count / 2; i < count; i++) {
        items[i] = count - i - 1;
    }
}

int cmpfunc(const void *a, const void *b) {
    return (*(int *)a - *(int *)b);
}

void shell(int *items, int count) {
    int i, j, gap, k;
    int x, a[5];

    a[0] = 9; a[1] = 5; a[2] = 3; a[3] = 2; a[4] = 1;

    for (k = 0; k < 5; k++) {
        gap = a[k];
        for (i = gap; i < count; ++i) {
            x = items[i];
            for (j = i - gap; (x < items[j]) && (j >= 0); j = j - gap)
                items[j + gap] = items[j];
            items[j + gap] = x;
        }
    }
}

void qs(int *items, int left, int right) {
    int i, j;
    int x, y;

    i = left; j = right;

    /* выбор компаранда */
    x = items[(left + right) / 2];

    do {
        while ((items[i] < x) && (i < right)) i++;
        while ((x < items[j]) && (j > left)) j--;

        if (i <= j) {
            y = items[i];
            items[i] = items[j];
            items[j] = y;
            i++; j--;
        }
    } while (i <= j);

    if (left < j) qs(items, left, j);
    if (i < right) qs(items, i, right);
}
