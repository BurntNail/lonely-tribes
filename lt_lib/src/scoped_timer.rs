use std::time::Instant;

//Thanks Cherno!
///A timer that sends a message of how long it took when it goes out of scope
pub struct ScopedTimer {
    message: String,
    instant: Instant,
}

impl ScopedTimer {
    ///Used for creating a new scoped timer
    ///
    ///# Examples
    ///```rust
    /// use lonely_tribes_lib::scoped_timer::ScopedTimer;
    /// {
    ///    let _st = ScopedTimer::new("Rendering took {}".to_string());
    ///
    ///    //do expensive op that takes 2 seconds
    /// }
    /// //prints "Rendering took 2s"
    /// ```
    pub fn new(message: String) -> Self {
        Self {
            message,
            instant: Instant::now(),
        }
    }
}

impl Drop for ScopedTimer {
    fn drop(&mut self) {
        let msg = self
            .message
            .replace("{}", &format!("{:?}", self.instant.elapsed()));
        log::info!("{}", msg);
    }
}
