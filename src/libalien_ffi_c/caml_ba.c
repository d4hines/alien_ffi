
#include <caml/mlvalues.h>
#include <caml/bigarray.h>

#include "caml_raw_pointer.h"

#ifndef Caml_ba_layout_val
/* Caml_ba_layout_val was introduced when the representation of layout
   values changed from an integer to a GADT.  Up to that point the 
   OCaml values c_layout and fortran_layout had the same values as
   the C constants CAML_BA_C_LAYOUT and CAML_BA_FORTRAN_LAYOUT */
#define Caml_ba_layout_val(v) (Int_val(v))
#endif

// Convenience function to allocate a u8 bigarray with C layout.
value caml_ba_alloc_u8(void* data, int ndims, long* dims)
{
  int flags = CAML_BA_UINT8 | CAML_BA_C_LAYOUT;
  return caml_ba_alloc(flags, ndims, data, dims);
}
