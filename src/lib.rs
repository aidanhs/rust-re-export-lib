// Demonstration of https://github.com/rust-lang/rust/issues/110624
// i.e. even with +whole-archive, re-exporting requires no_mangle
#[link(name="ext_issue110624", kind="static", modifiers = "+whole-archive")]
extern "C" {
    #[no_mangle]
    pub fn foo_issue110624_with_no_mangle();
    pub fn foo_issue110624_without_no_mangle();
}
#[link(name="ext_issue110624_2", kind="static", modifiers = "+whole-archive")]
extern "C" {
    pub fn foo_issue110624_2_without_no_mangle();
}

// Demonstration of https://github.com/rust-lang/rfcs/pull/3556
#[link(name="ext_rfc3556", kind="static")]
extern "C" {
    // FUNCTIONS

    // Should be re-exported
    #[no_mangle]
    pub fn foo_rfc3556_pub_with_no_mangle();
    // Should warn and not be re-exported
    #[no_mangle]
    fn foo_rfc3556_priv_with_no_mangle();

    // Should not be re-exported
    pub fn foo_rfc3556_pub_without_no_mangle();
    // Should not be re-exported
    fn foo_rfc3556_priv_without_no_mangle();

    // STATICS

    // Should be re-exported
    #[no_mangle]
    pub static mut foo_rfc3556_global_pub_with_no_mangle: std::ffi::c_int;
    // Should warn and not be re-exported
    #[no_mangle]
    static mut foo_rfc3556_global_priv_with_no_mangle: std::ffi::c_int;

    // Should not be re-exported
    pub static mut foo_rfc3556_global_pub_without_no_mangle: std::ffi::c_int;
    // Should not be re-exported
    static mut foo_rfc3556_global_priv_without_no_mangle: std::ffi::c_int;
}

// Demonstration of normal symbol export
#[no_mangle]
pub extern "C" fn foo_pub_no_mangle_rust_fn() {}
// no_mangle on extern rust functions always marks the symbol as globally visible
#[no_mangle]
extern "C" fn foo_priv_no_mangle_rust_fn() {}

// Make sure all the imports are used
#[no_mangle]
pub unsafe extern "C" fn x() -> std::ffi::c_int {
    foo_issue110624_with_no_mangle();
    foo_issue110624_without_no_mangle();
    foo_issue110624_2_without_no_mangle();

    foo_rfc3556_pub_with_no_mangle();
    foo_rfc3556_priv_with_no_mangle();

    foo_rfc3556_pub_without_no_mangle();
    foo_rfc3556_priv_without_no_mangle();

    foo_rfc3556_global_pub_with_no_mangle +
        foo_rfc3556_global_priv_with_no_mangle +
        foo_rfc3556_global_pub_without_no_mangle +
        foo_rfc3556_global_priv_without_no_mangle
}
