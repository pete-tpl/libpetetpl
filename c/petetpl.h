#ifndef _H_PETETPL_
#define _H_PETETPL_

#define PETETPL_PARAM_TYPE_INT 1
#define PETETPL_PARAM_TYPE_FLOAT 2
#define PETETPL_PARAM_TYPE_STRING 5

struct petetpl_render_result {
    char* output;
    int error_code;
};

struct petetpl_param {
    char* name;
    unsigned short int param_type;
    double value_float;
    long long int value_int;
    char* value_string;
};

void petetpl_init();
int petetpl_create_new();
struct petetpl_render_result* petetpl_render(int, const char*, int, struct petetpl_param*);

#endif