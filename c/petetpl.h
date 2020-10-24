#ifndef _H_PETETPL_
#define _H_PETETPL_

struct petetpl_render_result {
    char* output;
    int error_code;
};

void (*petetpl_init)() = NULL;
int (*petetpl_create_new)() = NULL;
struct petetpl_render_result* (*petetpl_render)(int, const char*) = NULL;

#endif