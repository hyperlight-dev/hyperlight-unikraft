#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    const char *my_var = getenv("MY_VAR");
    const char *debug = getenv("DEBUG");
    const char *greeting = getenv("GREETING");
    printf("MY_VAR=%s\n", my_var ? my_var : "");
    printf("DEBUG=%s\n", debug ? debug : "");
    printf("GREETING=%s\n", greeting ? greeting : "");
    return 0;
}
