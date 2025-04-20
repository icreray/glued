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
