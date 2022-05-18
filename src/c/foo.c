#include<stdio.h>
#include<stdint.h>

void foo()
{
    printf("foo!!\n");
}

typedef struct Hoge {
    int32_t x;
    int32_t y;
} Hoge;

void hoge(Hoge* h) {
    printf("x: %d, y: %d\n", h->x, h->y);
}

Hoge fuga() {
    Hoge h = {
        x: 1,
        y: 5
    };

    return h;
}