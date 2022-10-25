TARGET = libalien_ffi_rs

$(TARGET): target/release/libalien_ffi_rs.a target/release/libalien_ffi_rs.so

target/release/libalien_ffi_rs.a:
	cargo build --offline --release

target/release/libalien_ffi_rs.so:
	cargo build --offline --release

clean:
	cargo clean
