#[allow(unused_macros)]
macro_rules! flags {
    ($name:ident, $($flag:ident),*) => {
        pub struct $name {
            $(
                $flag: bool,
            )*
        }

        let my_flag = $name {
            $(
                $flag: false,
            )*
        }

        my_flag
    };
}
