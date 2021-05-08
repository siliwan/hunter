use std::io::Result;

/// If the socket is not requested, this function is called.
/// # What it does
/// It looks into the other arguments and reports appropriate
/// configurations.
pub fn start_no_socket(args: Vec<String>) -> Result<()> {
    Ok(())
}
