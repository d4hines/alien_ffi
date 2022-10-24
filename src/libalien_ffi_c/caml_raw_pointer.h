
#ifndef CAML_RAW_POINTER_H
#define CAML_RAW_POINTER_H

#include <caml/mlvalues.h>
#include <caml/alloc.h>
#include <stdint.h>

#define OF_PTR(P) caml_copy_nativeint((intptr_t)P)
#define TO_PTR(I) ((void*)Nativeint_val(I))

#define ADDR_OF_CAMLPTR(P) TO_PTR(Field(P, 1))

#endif
