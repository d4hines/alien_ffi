
#include <caml/mlvalues.h>
#include <caml/custom.h>
#include <stdio.h>

#define Custom_val_address(v) (*((void **) Data_custom_val(v)))

// Default custom operations (macro -> function)

int caml_compare_default(value a, value b) {
  fprintf(stderr, "error: compare not implemented\n");
  return 0;
}

int caml_compare_ext_default(value a, value b) {
  fprintf(stderr, "error: compare_ext not implemented");
  return 0;
}

int caml_hash_default(value v) {
  fprintf(stderr, "error: hash not implemented");
  return 0;
}

void caml_serialize_default(value v, int* bsize_32, int* bsize_64) {
  fprintf(stderr, "error: serialize not implemented");
}

int caml_deserialize_default(void* dst) {
  fprintf(stderr, "error: deserialize not implemented");
  return 0;
}
  

// Accessor / Mutators

void* get_custom_data(value v)
{
  return Data_custom_val(v);
}

void set_custom_data(value v, void* p)
{
  Custom_val_address(v) = p;
}

void
set_finalize(struct custom_operations* ops, void (*finalize)(value v))
{
  ops->finalize = finalize;
}

