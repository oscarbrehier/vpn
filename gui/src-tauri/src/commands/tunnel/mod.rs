pub mod metadata;
pub mod tunnel;
pub mod app_filter;

use tokio::sync::{Mutex, mpsc};
pub use tunnel::*;

pub struct RedirectionState {
	pub filter_rx: Mutex<Option<mpsc::UnboundedSender<Vec<u32>>>>
}

impl Default for RedirectionState {
	fn default() -> Self {
		Self { filter_rx: Mutex::new(None) }
	}
}