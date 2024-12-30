use crate::error_hook;
use crate::ArwenApiImpl;
use numbat_wasm::api::ErrorApi;

impl ErrorApi for ArwenApiImpl {
	#[inline]
	fn signal_error(&self, message: &[u8]) -> ! {
		error_hook::signal_error(message)
	}
}
