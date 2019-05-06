#[macro_export]
macro_rules! path {
    ( $( $x:expr ),* ) => {
        Path::new(".")
            $(
                .join($x.to_string())
            )*
    }
}
