pub mod error;

use tokio::io;

/// Possible return values of callback.
pub enum CallbackResult {
    /// Wait for next clipboard change.
    Next,
    /// Stop handling messages.
    Stop,
    /// Special variant to propagate IO Error from callback.
    StopWithError(io::Error)
}

pub trait ClipboardListener {
    /// Callback to call on clipboard change
    fn on_change(&mut self) -> CallbackResult;

    /// Callback to call on when error happens
    fn on_error(&mut self, error: io::Error) -> CallbackResult {
        CallbackResult::StopWithError(error)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clipboard_listener() {

    }
}
