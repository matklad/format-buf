/// A drop-in replacement for `std::format!`, which can optionally accept a an
/// existing `String` buffer.
///
/// ```rust
/// use format_buf::format;
///
/// let mut buf = format!("Roses are {},\n", "red");
/// let () = format!(buf, "Violets are {}.", "blue");
/// assert_eq!(buf, "\
///     Roses are red,\n\
///     Violets are blue.\
/// ")
/// ```
#[macro_export]
macro_rules! format {
    () => (::std::format!());
    ($lit:literal $($arg:tt)*) => (::std::format!($lit $($arg)*));
    ($buf:expr, $lit:literal $($arg:tt)*) => {
        { use ::std::fmt::Write as _; let _ = ::std::write!($buf, $lit $($arg)*); }
    };
}
