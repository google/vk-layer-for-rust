use crate::bindings::{buffer_handle_t, cros_gralloc_handle, cros_gralloc_handle_t};

const CROS_GRALLOC_MAGIC: u32 = 0xABCDDCBA;

#[deny(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn cros_gralloc_convert_handle<'a>(
    handle: buffer_handle_t,
) -> Option<&'a cros_gralloc_handle> {
    let handle = handle as cros_gralloc_handle_t;
    let handle = match unsafe { handle.as_ref() } {
        Some(handle) => handle,
        None => return None,
    };
    if handle.magic != CROS_GRALLOC_MAGIC {
        return None;
    }
    Some(handle)
}
