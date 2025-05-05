use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{DeriveInput, Index};

use crate::utils::{self, paths::*};

pub fn expand_derive(ast: DeriveInput) -> syn::Result<TokenStream2> {
	let fields = utils::get_struct_fields(&ast.data)?;

	let generics = ast.generics;
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
	let struct_name = &ast.ident;

	let crate_name = glued_crate_name();
	let with_trait = with_trait(&crate_name);
	let modular_app_trait = modular_app_trait(&crate_name);
	let schedule_label_trait = schedule_label_trait(&crate_name);
	let system_trait = schedule_system_trait(&crate_name);

	let with_impls = fields.iter().enumerate().map(|(i, field)| {
		let name = field.ident.as_ref().map_or(
			Index::from(i).to_token_stream(),
			Ident::to_token_stream
		);

		let field_type = &field.ty;
		quote! {
			impl #impl_generics #with_trait<#field_type> for #struct_name #ty_generics #where_clause {
				#[inline(always)]
				fn get(&self) -> &#field_type {&self.#name}
				#[inline(always)]
				fn get_mut(&mut self) -> &mut #field_type {&mut self.#name}
			}
		}
	});

	let calls = fields.iter().map(|field| {
		let field_type = &field.ty;
		quote! { <#field_type as #system_trait<L, Self>>::run(self);}
	}).collect::<Vec<_>>();

	Ok(quote! {
		unsafe impl #impl_generics #modular_app_trait for #struct_name #ty_generics #where_clause {
			fn run<L: #schedule_label_trait>(&mut self) {
				#(#calls)*
			}
		}

		#(#with_impls)*
	})
}
