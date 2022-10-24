TARGET = libalien_ffi_rs

$(TARGET): target/debug/libalien_ffi_rs.a target/debug/libalien_ffi_rs.so

target/debug/libalien_ffi_rs.a:
	cargo build --offline

target/debug/libalien_ffi_rs.so:
	cargo build --offline

clean:
	cargo clean
