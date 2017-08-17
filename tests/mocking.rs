#![feature(const_fn, proc_macro)]

extern crate mocktopus;

mod mocking_fns;
mod mocking_methods;
mod mocking_trait_defaults;
mod mocking_traits;

use mocktopus::macros::*;
use mocktopus::mocking::*;
use mocktopus::utils::*;
#[allow(unused_imports)] //Linter error
use std::ascii::AsciiExt;
use std::fmt::Display;

mod mock_safe {
    use super::*;

    #[mockable]
    pub fn no_args_returns_str() -> &'static str {
        "not mocked"
    }

    #[test]
    fn when_not_mocked_then_returns_not_mocked() {
        assert_eq!("not mocked", no_args_returns_str());
    }

    #[test]
    fn when_mocked_then_returns_mocked() {
        no_args_returns_str.mock_safe(|| MockResult::Return("mocked"));

        assert_eq!("mocked", no_args_returns_str());
    }
}

mod mocks_do_not_leak_between_tests {
    use super::*;

    #[mockable]
    pub fn no_args_returns_str() -> &'static str {
        "not mocked"
    }

    macro_rules! generate_tests {
        ($($fn_name:ident),+) => {
            $(
                #[test]
                fn $fn_name() {
                    assert_eq!("not mocked", no_args_returns_str(), "function was mocked before mocking");

                    unsafe {
                        no_args_returns_str.mock_raw(|| MockResult::Return((stringify!($fn_name))));
                    }

                    assert_eq!(stringify!($fn_name), no_args_returns_str(), "mocking failed");
                }
            )+
        }
    }

    generate_tests!(t00, t01, t02, t03, t04, t05, t06, t07, t08, t09, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19);
    generate_tests!(t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31, t32, t33, t34, t35, t36, t37, t38, t39);
    generate_tests!(t40, t41, t42, t43, t44, t45, t46, t47, t48, t49, t50, t51, t52, t53, t54, t55, t56, t57, t58, t59);
    generate_tests!(t60, t61, t62, t63, t64, t65, t66, t67, t68, t69, t70, t71, t72, t73, t74, t75, t76, t77, t78, t79);
    generate_tests!(t80, t81, t82, t83, t84, t85, t86, t87, t88, t89, t90, t91, t92, t93, t94, t95, t96, t97, t98, t99);
}

mod mocking_generic_over_a_type_with_lifetime_mocks_all_lifetime_variants {
    use super::*;
    use std::fmt::Display;

    #[mockable]
    fn function<T: Display>(generic: T) -> String {
        format!("not mocked {}", generic)
    }

    static STATIC_CHAR: char = 'S';

    #[test]
    fn all_lifetime_variants_get_mocked() {
        unsafe {
            function::<&char>.mock_raw(|c| MockResult::Return(format!("mocked {}", c)));
        }
        let local_char = 'L';

        assert_eq!("mocked L", function(&local_char));
        assert_eq!("mocked S", function(&STATIC_CHAR));
        assert_eq!("not mocked 3", function(&3));
    }
}

mod mocking_generic_over_a_reference_does_not_mock_opposite_mutability_variant {
    use super::*;
    use std::fmt::Display;

    #[mockable]
    fn function<T: Display>(generic: T) -> String {
        format!("not mocked {}", generic)
    }

    #[test]
    fn mocking_for_ref_does_not_mock_for_mut_ref() {
        unsafe {
            function::<&char>.mock_raw(|c| MockResult::Return(format!("mocked {}", c)));
        }

        assert_eq!("mocked R", function(&'R'));
        assert_eq!("not mocked M", function(&mut 'M'));
    }

    #[test]
    fn mocking_for_mut_ref_does_not_mock_for_ref() {
        unsafe {
            function::<&mut char>.mock_raw(|c| MockResult::Return(format!("mocked {}", c)));
        }

        assert_eq!("not mocked R", function(&'R'));
        assert_eq!("mocked M", function(&mut 'M'));
    }
}
