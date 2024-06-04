#![allow(improper_ctypes)]
#[link(name = "wal_3dxp", kind = "dylib")]
extern "C" {
	pub fn hook_symbol(symbol: &str, func: *const ()) -> *const ();
	pub fn get_symbol(symbol: &str) -> *mut ();
	pub fn hook(address: *mut (), func: *const ()) -> *const ();
	pub fn write_memory(address: *mut (), data: &[u8]);
}
