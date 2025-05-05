use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::utils::paths::*;

pub(crate) fn expand_derive(input: DeriveInput) -> TokenStream2 {
	let crate_name = glued_crate_name();
	let module_trait = module_trait(&crate_name);

	let ident = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	quote! {
		unsafe impl #impl_generics #module_trait for #ident #ty_generics #where_clause {}
	}
}

