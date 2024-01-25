# re-exporting symbols from a linked staticlib in a rust cdylib

Intended for use on Linux

script.sh will:
 - nuke build directories (`build` and `target`)
 - create small staticlibs (.a) from each `ext_*.c` file, each containing a few distinct symbols (`foo_*`)
 - build the cdylib (.so) rust project which links against `build/libext_*.a`
 - run `nm` against the created cdylib, grepping for `foo` (from `ext_*.c`) and `bar` (from lib.rs)

Running script.sh will show the following

```
$ ./script.sh
ar: creating build/libext_issue110624.a
ar: creating build/libext_issue110624_2.a
ar: creating build/libext_rfc3556.a
   Compiling x v0.1.0 (/home/aidanhs/rfc-no-mangle/rust-re-export-lib)
warning: `#[no_mangle]` has no effect on a foreign function
 --> src/lib.rs:5:5
  |
5 |     #[no_mangle]
  |     ^^^^^^^^^^^^ help: remove this attribute
6 |     pub fn foo_issue110624_with_no_mangle();
  |     ---------------------------------------- foreign function
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: symbol names in extern blocks are not mangled
  = note: `#[warn(unused_attributes)]` on by default

[...snip a number of similar warnings...]

warning: `x` (lib) generated 5 warnings (run `cargo fix --lib -p x` to apply 5 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
=== DYNAMIC SYMBOLS ===
0000000000006f35 T foo_issue110624_with_no_mangle
0000000000006df0 T foo_priv_no_mangle_rust_fn
0000000000006de0 T foo_pub_no_mangle_rust_fn
0000000000054008 D foo_rfc3556_global_pub_with_no_mangle
0000000000006f56 T foo_rfc3556_pub_with_no_mangle
=== NORMAL SYMBOLS ===
0000000000006f4b t foo_issue110624_2_without_no_mangle
0000000000006f35 T foo_issue110624_with_no_mangle
0000000000006f40 t foo_issue110624_without_no_mangle
0000000000006df0 T foo_priv_no_mangle_rust_fn
0000000000006de0 T foo_pub_no_mangle_rust_fn
000000000005400c d foo_rfc3556_global_priv_with_no_mangle
0000000000054014 d foo_rfc3556_global_priv_without_no_mangle
0000000000054008 D foo_rfc3556_global_pub_with_no_mangle
0000000000054010 d foo_rfc3556_global_pub_without_no_mangle
0000000000006f61 t foo_rfc3556_priv_with_no_mangle
0000000000006f77 t foo_rfc3556_priv_without_no_mangle
0000000000006f56 T foo_rfc3556_pub_with_no_mangle
0000000000006f6c t foo_rfc3556_pub_without_no_mangle
```

Cross referencing this with `src/lib.rs` you will see that:
 - public symbols with `#[no_mangle]` (`*_with_no_mangle`) are re-exported, contrary to rustc's claims that it has no effect
 - this re-exporting works for both `fn` and `static` items
 - removing either public visibility or `#[no_mangle]` causes the symbol to not be exported
