#include <cstdio>

int ret_int() { return 42; }

int main()
{
    int x = ret_int();

    printf("Int is: %d", x);

    return 0;
}
