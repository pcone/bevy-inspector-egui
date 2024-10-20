use std::any::TypeId;

use bevy_reflect::{GetTypeRegistration, Reflect};
use bevy_utils::all_tuples;

pub trait ComponentFilter {
    fn matches(component: &dyn Reflect) -> bool;
    fn matches_type<T: Reflect + GetTypeRegistration>() -> bool;
    fn matches_type_id(type_id: &TypeId) -> bool;
}
pub struct Exclude<T> {
    _marker: std::marker::PhantomData<T>,
}
pub struct Include<T> {
    _marker: std::marker::PhantomData<T>,
}
macro_rules! impl_tuple_component_filter_exclude {
    ($(#[$meta:meta])* $($name: ident),*) => {
        $(#[$meta])*
        impl<$($name: Reflect + GetTypeRegistration),*> ComponentFilter for Exclude<($($name,)*)> {
            fn matches(component: &dyn Reflect) -> bool {
                true $(&& !component.is::<$name>())*
            }
            fn matches_type<T: Reflect + GetTypeRegistration>() -> bool {
                true $(&& !$name::get_type_registration().type_id().eq(&T::get_type_registration().type_id()))*
            }
            fn matches_type_id(type_id: &TypeId) -> bool {
                true $(&& !$name::get_type_registration().type_id().eq(type_id))*
            }
        }
    };
}
all_tuples!(
    impl_tuple_component_filter_exclude,
    0,
    15,
    F
);

macro_rules! impl_tuple_component_filter_include {
    ($(#[$meta:meta])* $($name: ident),*) => {
        $(#[$meta])*
        impl<$($name: Reflect + GetTypeRegistration),*> ComponentFilter for Include<($($name,)*)> {
            fn matches(component: &dyn Reflect) -> bool {
                false $(|| component.is::<$name>())*
            }
            fn matches_type<T: Reflect + GetTypeRegistration>() -> bool {
                false $(|| $name::get_type_registration().type_id().eq(&T::get_type_registration().type_id()))*
            }
            fn matches_type_id(type_id: &TypeId) -> bool {
                false $(|| $name::get_type_registration().type_id().eq(type_id))*
            }
        }
    };
}
all_tuples!(
    impl_tuple_component_filter_include,
    0,
    15,
    F
);

macro_rules! impl_tuple_component_filter {
    ($(#[$meta:meta])* $($name: ident),*) => {
        $(#[$meta])*
        impl<$($name: ComponentFilter),*> ComponentFilter for ($($name,)*) {
            fn matches(component: &dyn Reflect) -> bool {
                true $(&& $name::matches(component))*
            }

            fn matches_type<T: Reflect + GetTypeRegistration>() -> bool {
                true $(&& $name::matches_type::<T>())*
            }

            fn matches_type_id(type_id: &TypeId) -> bool {
                true $(&& $name::matches_type_id(type_id))*
            }
        }
    };
}

all_tuples!(
    impl_tuple_component_filter,
    0,
    15,
    F
);
