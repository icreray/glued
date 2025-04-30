use syn::{
	Data, DataStruct, Field, Fields, Visibility, punctuated::Punctuated, token::Comma
};

macro_rules! spanned_error {
    ($message:literal) => {Err(syn::Error::new(proc_macro2::Span::call_site(), $message))};
	($span:expr, $message:expr) => {Err(syn::Error::new_spanned($span, $message))};
}
pub(crate) use spanned_error;

pub(crate) fn get_struct_fields(data: &Data) -> syn::Result<&Punctuated<Field, Comma>> {
	match data {
		Data::Struct(DataStruct {
			fields: Fields::Named(fields),
			..
		}) => Ok(&fields.named),
		Data::Struct(DataStruct {
			fields: Fields::Unnamed(fields),
			..
		}) => Ok(&fields.unnamed),
		_ => spanned_error!("Only structs are supported")
	}
}

pub(crate) trait VisibilityExt {
	fn is_public(&self) -> bool;
}
impl VisibilityExt for Visibility {
	fn is_public(&self) -> bool {
		match self {
			Visibility::Public(_) => true,
			_ => false
		}
	}
}

pub(crate) mod paths {
	use proc_macro2::TokenStream;
	use proc_macro_crate::{crate_name, FoundCrate};
	use quote::quote;
	use syn::parse_str;

	pub fn glued_crate_name() -> TokenStream {
		let found = crate_name("glued")
			.expect("Failed to find crate \'glued\'");
		match found {
			FoundCrate::Itself => quote! {crate},
			FoundCrate::Name(name) => parse_str(&name)
				.expect("Failed to parse crate name into tokens")
		}
	}

	pub fn modular_app_trait(crate_name: &TokenStream) -> TokenStream {
		quote! {#crate_name::ModularApp}
	}

	pub fn module_trait(crate_name: &TokenStream) -> TokenStream {
		quote! {#crate_name::module::Module}
	}

	pub fn with_trait(crate_name: &TokenStream) -> TokenStream {
		quote! {#crate_name::module::With}
	}
}
