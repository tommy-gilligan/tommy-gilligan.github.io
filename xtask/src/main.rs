#![feature(async_closure)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
// #![deny(clippy::absolute_paths)]
#![allow(clippy::alloc_instead_of_core)]
#![deny(clippy::allow_attributes)]
#![allow(clippy::allow_attributes_without_reason)]
#![allow(clippy::arithmetic_side_effects)]
#![deny(clippy::as_conversions)]
#![deny(clippy::as_underscore)]
#![deny(clippy::assertions_on_result_states)]
#![allow(clippy::big_endian_bytes)]
#![deny(clippy::clone_on_ref_ptr)]
#![deny(clippy::create_dir)]
#![deny(clippy::dbg_macro)]
#![deny(clippy::decimal_literal_representation)]
#![deny(clippy::default_numeric_fallback)]
#![deny(clippy::default_union_representation)]
#![allow(clippy::deref_by_slicing)]
#![deny(clippy::disallowed_script_idents)]
#![deny(clippy::else_if_without_else)]
#![deny(clippy::empty_drop)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::error_impl_error)]
#![allow(clippy::exhaustive_enums)]
#![allow(clippy::exhaustive_structs)]
#![deny(clippy::exit)]
#![allow(clippy::expect_used)]
#![deny(clippy::filetype_is_file)]
#![allow(clippy::float_arithmetic)]
#![deny(clippy::float_cmp_const)]
#![deny(clippy::fn_to_numeric_cast_any)]
#![deny(clippy::format_push_string)]
#![deny(clippy::get_unwrap)]
#![allow(clippy::host_endian_bytes)]
#![deny(clippy::if_then_some_else_none)]
#![deny(clippy::impl_trait_in_params)]
#![deny(clippy::implicit_return)]
#![deny(clippy::indexing_slicing)]
#![deny(clippy::inline_asm_x86_att_syntax)]
#![deny(clippy::inline_asm_x86_intel_syntax)]
#![deny(clippy::integer_division)]
#![deny(clippy::large_include_file)]
#![deny(clippy::let_underscore_must_use)]
#![deny(clippy::let_underscore_untyped)]
#![allow(clippy::little_endian_bytes)]
#![deny(clippy::lossy_float_literal)]
#![deny(clippy::map_err_ignore)]
#![deny(clippy::mem_forget)]
#![allow(clippy::min_ident_chars)]
#![allow(clippy::missing_assert_message)]
#![allow(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_enforced_import_renames)]
#![allow(clippy::missing_inline_in_public_items)]
#![deny(clippy::missing_trait_methods)]
#![deny(clippy::mixed_read_write_in_expression)]
#![deny(clippy::mod_module_files)]
#![deny(clippy::modulo_arithmetic)]
#![deny(clippy::multiple_inherent_impl)]
#![allow(clippy::multiple_unsafe_ops_per_block)]
#![deny(clippy::mutex_atomic)]
#![deny(clippy::needless_raw_strings)]
#![deny(clippy::non_ascii_literal)]
#![allow(clippy::panic)]
#![deny(clippy::panic_in_result_fn)]
#![deny(clippy::partial_pub_fields)]
#![deny(clippy::pattern_type_mismatch)]
#![deny(clippy::print_stderr)]
#![allow(clippy::print_stdout)]
#![deny(clippy::pub_use)]
#![allow(clippy::pub_with_shorthand)]
#![allow(clippy::needless_return)]
#![deny(clippy::pub_without_shorthand)]
#![allow(clippy::question_mark_used)]
#![deny(clippy::rc_buffer)]
#![deny(clippy::rc_mutex)]
#![deny(clippy::redundant_type_annotations)]
#![allow(clippy::ref_patterns)]
#![deny(clippy::rest_pat_in_fully_bound_structs)]
#![deny(clippy::same_name_method)]
#![allow(clippy::self_named_module_files)]
#![deny(clippy::semicolon_inside_block)]
#![allow(clippy::semicolon_outside_block)]
#![allow(clippy::separated_literal_suffix)]
#![allow(clippy::shadow_reuse)]
#![allow(clippy::shadow_same)]
#![deny(clippy::shadow_unrelated)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::single_char_lifetime_names)]
#![deny(clippy::std_instead_of_alloc)]
#![deny(clippy::std_instead_of_core)]
#![deny(clippy::str_to_string)]
#![deny(clippy::string_add)]
#![deny(clippy::string_lit_chars_any)]
#![deny(clippy::string_slice)]
#![deny(clippy::string_to_string)]
#![deny(clippy::suspicious_xor_used_as_pow)]
#![deny(clippy::tests_outside_test_module)]
#![deny(clippy::todo)]
#![deny(clippy::try_err)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::unimplemented)]
#![deny(clippy::unnecessary_safety_comment)]
#![deny(clippy::unnecessary_safety_doc)]
#![deny(clippy::unnecessary_self_imports)]
#![deny(clippy::unneeded_field_pattern)]
#![deny(clippy::unreachable)]
#![deny(clippy::unseparated_literal_suffix)]
#![allow(clippy::unwrap_in_result)]
#![allow(clippy::unwrap_used)]
#![deny(clippy::use_debug)]
#![deny(clippy::verbose_file_reads)]
#![deny(clippy::wildcard_enum_match_arm)]

mod crawl;
mod generate;
mod screenshot;
mod serve;
mod visual_diff;
mod watch;

use clap::Parser;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(bin_name = "xtask")]
enum Cli {
    Crawl(crawl::Args),
    Generate(generate::Args),
    Screenshot(screenshot::Args),
    Serve(serve::Args),
    VisualDiff(visual_diff::Args),
    Watch(watch::Args),
}

#[tokio::main]
async fn main() {
    match Cli::parse() {
        Cli::Generate(args) => generate::generate(&args),
        Cli::Crawl(args) => crawl::crawl(&args).await,
        Cli::Serve(args) => serve::serve(&args).await,
        Cli::Watch(args) => watch::watch(&args).await,
        Cli::Screenshot(args) => screenshot::screenshot(&args).await,
        Cli::VisualDiff(args) => visual_diff::visual_diff(&args),
    }
}
