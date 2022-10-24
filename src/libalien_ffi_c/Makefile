TARGET = libalien_ffi_c

$(TARGET): libalien_ffi_c.a libalien_ffi_c.so

caml_ba.o: caml_ba.c
	gcc -c -o $@ $<

caml_values.o: caml_values.c
	gcc -c -o $@ $<

caml_custom.o: caml_custom.c
	gcc -c -o $@ $<

libalien_ffi_c.a: caml_ba.o caml_values.o caml_custom.o
	ar rcs libalien_ffi_c.a $^

libalien_ffi_c.so: caml_ba.o caml_values.o caml_custom.o
	gcc -shared -o libalien_ffi_c.so -fPIC $^

clean:
	rm -rf *.o *.a *.so
