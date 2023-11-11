#[link(name="ext", kind="static", modifiers = "+whole-archive")]
extern "C" {
    // Without no_mangle foo doesn't get re-exported
    #[no_mangle]
    pub fn foo();
}

#[no_mangle]
pub extern "C" fn bar() {}
