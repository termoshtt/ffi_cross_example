Example for cross compile with FFI
-----------------------

```
$ cross run --target aarch64-unknown-linux-gnu
 Compiling ffi_cross_example v0.1.0 (/project)
  Finished dev [unoptimized + debuginfo] target(s) in 0.56s
   Running `qemu-aarch64 /target/aarch64-unknown-linux-gnu/debug/ffi_cross_example`
Fooo: 114514
```
