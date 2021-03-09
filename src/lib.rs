#[doc(hidden)]
#[macro_export]
macro_rules! share_internal {
    ($type:ident, $this:expr; $( $variant:tt ),* => |$item:pat| $item_op:expr) => {{
        use self::$type::*;

        #[allow(unreachable_patterns)]
        match $this {
            $(
                $variant($item) => $item_op,
            )*
            _ => unreachable!(),
        }
        }};
}

#[macro_export]
macro_rules! share {
    ($type:ident, $this:expr; $( $variant:tt ),* => |$item:ident| $item_op:expr) => {
    share_internal!($type, $this; $( $variant ),* => |ref $item| $item_op)
    }
}

#[macro_export]
macro_rules! share_mut {
    ($type:ident, $this:expr; $( $variant:tt ),* => |$item:ident| $item_op:expr) => {
    share_internal!($type, $this; $( $variant ),* => |ref mut $item| $item_op)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
