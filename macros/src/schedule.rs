use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

use crate::utils::{paths::{glued_crate_name, schedule_label_trait}, spanned_error};

pub(crate) fn expand_derive(input: DeriveInput) -> syn::Result<TokenStream2> {
	if !is_unit_struct(&input.data) {
		return spanned_error!(input, "ScheduleLabel can only be derived for unit structs")
	}
	let crate_name = glued_crate_name();
	let schedule_label_trait = schedule_label_trait(&crate_name);

	let ident = input.ident;
	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

	Ok (quote! {
		unsafe impl #impl_generics #schedule_label_trait for #ident #ty_generics #where_clause {}
	})
}

fn is_unit_struct(data: &Data) -> bool {
	matches!(data, Data::Struct(DataStruct {fields: Fields::Unit, ..}))
}
