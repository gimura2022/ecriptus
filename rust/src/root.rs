#![deny(
    deprecated,
    dead_code,
    unused,
    unreachable_code,
    large_assignments,
    unstable_features,
    unused_imports,
    unused_mut,
    unused_variables,
    warnings,
    clippy::all,
    clippy::pedantic,
    clippy::expect_used,
    clippy::float_cmp,
    clippy::panic,
    clippy::shadow_unrelated,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::indexing_slicing,
    clippy::unwrap_in_result,
    clippy::verbose_file_reads,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::inefficient_to_string,
    clippy::large_enum_variant,
    clippy::manual_assert,
    clippy::map_entry,
    clippy::missing_enforced_import_renames,
    clippy::modulo_arithmetic,
    clippy::needless_pass_by_value,
    clippy::new_without_default,
    clippy::no_effect,
    clippy::panic_in_result_fn,
    clippy::range_plus_one,
    clippy::redundant_allocation,
    clippy::redundant_clone,
    clippy::redundant_field_names,
    clippy::redundant_pub_crate,
    clippy::single_match_else,
    clippy::string_add,
    clippy::string_to_string,
    clippy::too_many_arguments,
    clippy::trivial_regex,
    clippy::type_complexity,
    clippy::unimplemented,
    clippy::unnecessary_cast,
    clippy::unnecessary_wraps,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unused_self,
    clippy::useless_conversion,
    clippy::wildcard_enum_match_arm,
    clippy::write_with_newline,
    clippy::wrong_self_convention
)]

use {
    c_func::*,
    std::{
        ffi::CString,
        panic::{self, PanicHookInfo}
    }
};

mod c_func;

#[allow(non_snake_case, unsafe_code)]
#[no_mangle]
pub fn RS_Main()
{
    panic::set_hook(Box::new(|x: &PanicHookInfo| {
        unsafe {
            let string: CString = CString::new(x.to_string()).unwrap();

            COM_SetComErrno(-1);
            COM_DieNoFormat(string.as_ptr());
        }
    }));

    panic!("panic");
}