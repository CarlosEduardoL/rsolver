/// A type alias for a `Result` where the error type is a `String`.
pub type ResolverResult<T> = Result<T, String>;

/// A macro for creating errors with source file and line information.
#[macro_export]
macro_rules! transform_result {
    ($result:expr) => {{
        match $result {
            Ok(value) => Ok(value),
            Err(err) => Err(format!("({}:{}) {:?}", file!(), line!(), err))
        }
    }};
    ($msg:expr, $result:expr) => {{
        match $result {
            Ok(value) => Ok(value),
            Err(err) => Err(format!("({}:{}) {}: {:?}", file!(), line!(), $msg, err))
        }
    }};
}
