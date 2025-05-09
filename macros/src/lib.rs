mod module;
mod modular_app;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error};

#[proc_macro_derive(ModularApp)]
pub fn derive_modular_app(input: TokenStream) -> TokenStream {
	modular_app::expand_derive(parse_macro_input!(input))
		.unwrap_or_else(Error::into_compile_error).into()
}

#[proc_macro_attribute]
#[doc(hidden)]
pub fn dependencies(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn module_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
	module::expand_module_impl(
		parse_macro_input!(attr),
		parse_macro_input!(item)
	).unwrap_or_else(Error::into_compile_error).into()
}
