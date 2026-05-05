/// A collection of sized items that collected into a slice in an arbitrary
/// order.
pub struct ScatteredSlice<T: Ord> {
    _marker: core::marker::PhantomData<T>,
}

impl<T: Ord> ScatteredSlice<T> {
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

#[macro_export]
macro_rules! __slice {
    (gather $vis:vis $name:ident: $ty:ty) => {
        #[doc(hidden)]
        $crate::__support::ident_concat!((#[macro_export] macro_rules!) (__ $name __sorted_slice_private_macro__) ({
            ($passthru:tt) => {
                $crate::__slice!(@scatter $passthru);
            };
        }));

        $crate::__support::ident_concat!((#[doc(hidden)] $vis use) (__ $name __sorted_slice_private_macro__) (as $name;));

        $vis static $name: $crate::sorted_slice::ScatteredSortedSlice<$ty> = {
            $crate::__support::link_section::declarative::section!(
                #[section(no_macro)]
                pub static $name: $crate::__support::link_section::TypedSection<$ty>;
            );

            $crate::sorted_slice::ScatteredSortedSlice::new(
                $name.const_deref(),
            )
        };
    };
    (scatter $collection:ident => $vis:vis _: $ty:ty = $expr:expr) => {
        $collection ! (( $collection => $vis _: $ty = $expr ));
    };
    (scatter $collection:ident => $vis:vis $name:ident: $ty:ty = $expr:expr) => {
        compile_error!("scatter items must be anonymous");
    };
    (@scatter ($collection:ident => $vis:vis _: $ty:ty = $expr:expr)) => {
        const _: () = {
            $crate::__support::link_section::declarative::in_section!(
                #[in_section(unsafe, type = $ty, name = $collection)]
                pub static _: $ty = $expr;
            );
        };
    };
}

#[cfg(test)]
mod tests {
    __slice!(gather pub TEST_SLICE: u32);
    __slice!(scatter TEST_SLICE => pub _: u32 = 1);
    __slice!(scatter TEST_SLICE => pub _: u32 = 3);
    __slice!(scatter TEST_SLICE => pub _: u32 = 2);

    #[test]
    fn test_scattered_slice() {
        assert_eq!(TEST_SLICE.len(), 3);
        assert!(TEST_SLICE.contains(&1));
        assert!(TEST_SLICE.contains(&2));
        assert!(TEST_SLICE.contains(&3));
    }
}
