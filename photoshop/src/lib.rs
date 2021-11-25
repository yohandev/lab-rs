extern
{
    /// Kill execution of this WASM module with error message at `ptr`
    fn abort(ptr: *const u8, len: usize);
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32
{
    a + b
}

#[no_mangle]
pub fn set_panic_hook()
{
    std::panic::set_hook(Box::new(|info| unsafe
    {
        let err = info.to_string();

        abort(err.as_ptr(), err.len())
    }))
}