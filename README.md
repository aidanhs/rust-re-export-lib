# re-exporting symbols from a linked staticlib in a rust cdylib

Intended for use on Linux

script.sh will:
 - nuke build directories
 - create a small staticlib (.a) from `ext.c` containing one symbol (`foo`)
 - build the cdylib (.so) rust project which links against `build/libext.a`
 - run `nm` against the created cdylib, grepping for `foo` (from ext.c) and `bar` (from lib.rs)

Running script.sh will show the following

```
$ ./script.sh
ar: creating build/libext.a
   Compiling x v0.1.0 (/tmp/tmp.cr0ryf5b1D/rust-re-export-lib)
warning: `#[no_mangle]` has no effect on a foreign function
 --> src/lib.rs:4:5
  |
4 |     #[no_mangle]
  |     ^^^^^^^^^^^^ help: remove this attribute
5 |     pub fn foo();
  |     ------------- foreign function
  |
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
  = note: symbol names in extern blocks are not mangled
  = note: `#[warn(unused_attributes)]` on by default

warning: `x` (lib) generated 1 warning (run `cargo fix --lib -p x` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38s
0000000000001100 T bar
0000000000001101 T foo
```

If you then go into `src/lib.rs` and comment out the `#[no_mangle]` on line 4 (which is warned about above) and rerun script.sh you will see:

```
$ ./script.sh
ar: creating build/libext.a
   Compiling x v0.1.0 (/tmp/tmp.cr0ryf5b1D/rust-re-export-lib)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
0000000000001100 T bar
```

i.e. while rustc claims that the `#[no_mangle]` has no effect, removing it causes the `foo` symbol to not be re-exported.
