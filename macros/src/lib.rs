mod module;
mod modular_app;
mod schedule;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error};

#[proc_macro_derive(ModularApp)]
pub fn derive_modular_app(input: TokenStream) -> TokenStream {
	modular_app::expand_derive(parse_macro_input!(input))
		.unwrap_or_else(Error::into_compile_error).into()
}

#[proc_macro_derive(Module)]
pub fn derive_module(input: TokenStream) -> TokenStream {
	module::expand_derive(parse_macro_input!(input))
		.into()
}

#[proc_macro_derive(ScheduleLabel)]
pub fn derive_schedule_label(input: TokenStream) -> TokenStream {
	schedule::expand_derive(parse_macro_input!(input))
		.unwrap_or_else(Error::into_compile_error).into()
}

#[proc_macro_attribute]
#[doc(hidden)]
pub fn requires(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}
