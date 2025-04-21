use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{DeriveInput, Index};

use crate::utils;

pub fn expand_derive(ast: DeriveInput) -> syn::Result<TokenStream2> {
	let fields = utils::get_struct_fields(&ast.data)?;

	let generics = ast.generics;
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
	let struct_name = &ast.ident;

	let with_impls = fields.iter().enumerate().map(|(i, field)| {
		let name = field.ident.as_ref().map_or(
			Index::from(i).to_token_stream(),
			Ident::to_token_stream
		);

		let field_type = &field.ty;
		quote! {
			impl #impl_generics glued::module::With<#field_type> for #struct_name #ty_generics #where_clause {
				#[inline(always)]
				fn get(&self) -> &#field_type {&self.#name}
				#[inline(always)]
				fn get_mut(&mut self) -> &mut #field_type {&mut self.#name}
			}
		}
	});

	let update_calls = fields.iter().map(|field| {
		let field_type = &field.ty;
		quote! { #field_type::update(self); }
	});

	Ok(quote! {
		unsafe impl #impl_generics glued::ModularApp for #struct_name #ty_generics #where_clause {}
		impl #impl_generics #struct_name #ty_generics #where_clause {
			pub fn update(&mut self) {
				#(#update_calls)*
			}
		}
		#(#with_impls)*
		impl #impl_generics #struct_name #ty_generics #where_clause {
			#[inline(always)]
			pub fn get_module<M: glued::Module>(&self) -> &M
			where Self: glued::module::With<M> {
				glued::module::With::<M>::get(self)
			}
			#[inline(always)]
			pub fn get_module_mut<M: glued::Module>(&mut self) -> &mut M
			where Self: glued::module::With<M> {
				glued::module::With::<M>::get_mut(self)
			}
		}
	})
}
