
#include <caml/mlvalues.h>

// Primitive value macro exports

long int long_val(value l)
{
  return Long_val(l);
}

int int_val(value i)
{
  return Int_val(i);
}

long nativeint_val(value ni)
{
  return Nativeint_val(ni);
}

value val_long(long int l)
{
  return Val_long(l);
}

value val_int(int i)
{
  return Val_int(i);
}

value val_nativeint(long ni)
{
  return Val_long(ni);
}

value val_true()
{
  return Val_bool(1);
}

value val_false()
{
  return Val_bool(0);
}

value val_unit()
{
  return Val_unit;
}
