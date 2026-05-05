/// A collection of sized items that are available both via referenced slice and
/// via reference at the declaration site.
///
/// If the reference to the individual items is not required, a
/// [`ScatteredSlice`] may be used instead.
pub struct ScatteredReferencedSlice<T: Ord> {
    _marker: core::marker::PhantomData<T>,
}

impl<T: Ord> ScatteredReferencedSlice<T> {
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

#[repr(transparent)]
pub struct Ref<T> {
    t: T,
}

impl<T> Ref<T> {
    pub const fn new(t: T) -> Self {
        Self { t }
    }
}

impl<T> ::core::ops::Deref for Ref<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.t
    }
}

crate::ref_impl::__ref!(Ref<T>);

#[macro_export]
macro_rules! __referenced_slice {
    (gather $vis:vis $name:ident: $ty:ty) => {
        #[doc(hidden)]
        $crate::__support::ident_concat!((#[macro_export] macro_rules!) (__ $name __sorted_slice_private_macro__) ({
            ($passthru:tt) => {
                $crate::__referenced_slice!(@scatter $passthru);
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
    (scatter $collection:ident => $vis:vis $name:ident: $ty:ty = $expr:expr) => {
        $collection ! (( $collection => $vis $name: $ty = $expr ));
    };
    (@scatter ($collection:ident => $vis:vis $name:ident: $ty:ty = $expr:expr)) => {
        $crate::__support::link_section::declarative::in_section!(
            #[in_section(unsafe, type = $crate::referenced_slice::Ref<$ty>, name = $collection)]
            pub static $name: $crate::referenced_slice::Ref<$ty> = $crate::referenced_slice::Ref::new($expr);
        );
    };
}

#[cfg(test)]
mod tests {
    __referenced_slice!(gather pub TEST_REF_SLICE: u32);
    __referenced_slice!(scatter TEST_REF_SLICE => pub ITEM_A: u32 = 1);
    __referenced_slice!(scatter TEST_REF_SLICE => pub ITEM_B: u32 = 3);
    __referenced_slice!(scatter TEST_REF_SLICE => pub ITEM_C: u32 = 2);

    #[test]
    fn test_scattered_slice() {
        assert_eq!(TEST_REF_SLICE.len(), 3);
        assert!(TEST_REF_SLICE.contains(&1));
        assert!(TEST_REF_SLICE.contains(&2));
        assert!(TEST_REF_SLICE.contains(&3));

        assert_eq!(*ITEM_A, 1);
        assert_eq!(*ITEM_B, 3);
        assert_eq!(*ITEM_C, 2);
    }
}
