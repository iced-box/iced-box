mod status;
pub mod helpers;
mod manager;
mod styles;
mod overlay;

use status::Status;

pub use manager::Manager;

/// Provides the instance for creating toast alerts, this functionality has been abstracted into helper functions.
#[derive(Debug, Clone, Default)]
pub struct Toast {
    pub title: String,
    pub body: String,
    pub status: Status,
    pub with_close: bool,
}

impl Toast {
    /// By default, helper functions create the alert without a body, with just the title, it is possible to add a body through this method.
    pub fn body(&mut self, body: &str) -> Self {
        self.body = body.to_string();

        self.clone()
    }

    /// By default, helper functions create the alert without a close button, it is possible to add a close button through this method.
    pub fn with_close(mut self) -> Self {
        self.with_close = true;
        self
    }   
}
