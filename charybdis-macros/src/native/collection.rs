use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_str;

use charybdis_parser::fields::CharybdisFields;
use charybdis_parser::traits::CharybdisMacroArgs;

use crate::traits::fields::FieldsQuery;
use crate::traits::tuple::FieldsAsTuple;

pub(crate) fn push_to_collection_consts(ch_args: &CharybdisMacroArgs, fields: &CharybdisFields) -> TokenStream {
    let queries: Vec<TokenStream> = fields
        .db_fields
        .iter()
        .filter_map(|field| {
            if !field.is_collection() {
                return None;
            }

            let query_str = format!(
                "UPDATE {} SET {} = {} + ? WHERE {}",
                ch_args.table_name(),
                field.name,
                field.name,
                fields.primary_key_fields.where_placeholders(),
            );

            let const_name = format!("PUSH_{}_QUERY", field.name.to_uppercase());
            let const_name: TokenStream = parse_str::<TokenStream>(&const_name).unwrap();

            let expanded = quote! {
                pub const #const_name: &'static str = #query_str;
            };

            Some(expanded)
        })
        .collect();

    let expanded = quote! {
        #(#queries)*
    };

    expanded
}

pub(crate) fn push_to_collection_consts_if_exists(
    ch_args: &CharybdisMacroArgs,
    fields: &CharybdisFields,
) -> TokenStream {
    let queries: Vec<TokenStream> = fields
        .db_fields
        .iter()
        .filter_map(|field| {
            if !field.is_collection() {
                return None;
            }

            let query_str = format!(
                "UPDATE {} SET {} = {} + ? WHERE {} IF EXISTS",
                ch_args.table_name(),
                field.name,
                field.name,
                fields.primary_key_fields.where_placeholders(),
            );

            let const_name = format!("PUSH_{}_IF_EXISTS_QUERY", field.name.to_uppercase());
            let const_name: TokenStream = parse_str::<TokenStream>(&const_name).unwrap();

            let expanded = quote! {
                pub const #const_name: &'static str = #query_str;
            };

            Some(expanded)
        })
        .collect();

    let expanded = quote! {
        #(#queries)*
    };

    expanded
}

pub(crate) fn pull_from_collection_consts(ch_args: &CharybdisMacroArgs, fields: &CharybdisFields) -> TokenStream {
    let queries: Vec<TokenStream> = fields
        .db_fields
        .iter()
        .filter_map(|field| {
            if !field.is_collection() {
                return None;
            }

            let query_str = format!(
                "UPDATE {} SET {} = {} - ? WHERE {}",
                ch_args.table_name(),
                field.name,
                field.name,
                fields.primary_key_fields.where_placeholders(),
            );

            let const_name = format!("PULL_{}_QUERY", field.name.to_uppercase());
            let const_name: TokenStream = parse_str::<TokenStream>(&const_name).unwrap();

            let expanded = quote! {
                pub const #const_name: &'static str = #query_str;
            };

            Some(expanded)
        })
        .collect();

    let expanded = quote! {
        #(#queries)*
    };

    expanded
}

pub(crate) fn pull_from_collection_consts_if_exists(
    ch_args: &CharybdisMacroArgs,
    fields: &CharybdisFields,
) -> TokenStream {
    let queries: Vec<TokenStream> = fields
        .db_fields
        .iter()
        .filter_map(|field| {
            if !field.is_collection() {
                return None;
            }

            let query_str = format!(
                "UPDATE {} SET {} = {} - ? WHERE {} IF EXISTS",
                ch_args.table_name(),
                field.name,
                field.name,
                fields.primary_key_fields.where_placeholders(),
            );

            let const_name = format!("PULL_{}_IF_EXISTS_QUERY", field.name.to_uppercase());
            let const_name: TokenStream = parse_str::<TokenStream>(&const_name).unwrap();

            let expanded = quote! {
                pub const #const_name: &'static str = #query_str;
            };

            Some(expanded)
        })
        .collect();

    let expanded = quote! {
        #(#queries)*
    };

    expanded
}

pub(crate) fn push_to_collection_methods(fields: &CharybdisFields) -> TokenStream {
    let push_to_collection_rules: Vec<TokenStream> = fields
        .db_fields
        .iter()
        .filter_map(|field| {
            if !field.is_collection() {
                return None;
            }

            let push_to_query_str = format!("Self::PUSH_{}_QUERY", field.name.to_uppercase());
            let push_to_query = parse_str::<TokenStream>(&push_to_query_str).unwrap();
            let fun_name_str = format!("push_{}", field.name);
            let fun_name = parse_str::<TokenStream>(&fun_name_str).unwrap();
            let types = fields.primary_key_fields.types();
            let values = fields.primary_key_fields.values();

            let expanded = quote! {
                pub fn #fun_name<V: charybdis::scylla::SerializeValue>(
                    &self,
                    value: V
                ) -> charybdis::query::CharybdisQuery<(V, #(#types),*), Self, charybdis::query::ModelMutation> {
                    charybdis::query::CharybdisQuery::new(
                        #push_to_query,
                        charybdis::query::QueryValue::Owned((value, #(#values),*)),
                    )
                }
            };

            Some(expanded)
        })
        .collect();

    let expanded = quote! {
        #(#push_to_collection_rules)*
    };

    expanded
}

pub(crate) fn push_to_collection_methods_if_exists(fields: &CharybdisFields) -> TokenStream {
    let push_to_collection_rules: Vec<TokenStream> = fields
        .db_fields
        .iter()
        .filter_map(|field| {
            if !field.is_collection() {
                return None;
            }

            let push_to_query_str = format!("Self::PUSH_{}_IF_EXISTS_QUERY", field.name.to_uppercase());
            let push_to_query = parse_str::<TokenStream>(&push_to_query_str).unwrap();
            let fun_name_str = format!("push_{}_if_exists", field.name);
            let fun_name = parse_str::<TokenStream>(&fun_name_str).unwrap();
            let types = fields.primary_key_fields.types();
            let values = fields.primary_key_fields.values();

            let expanded = quote! {
                pub fn #fun_name<V: charybdis::scylla::SerializeValue>(
                    &self,
                    value: V
                ) -> charybdis::query::CharybdisQuery<(V, #(#types),*), Self, charybdis::query::ModelMutation> {
                    charybdis::query::CharybdisQuery::new(
                        #push_to_query,
                        charybdis::query::QueryValue::Owned((value, #(#values),*)),
                    )
                }
            };

            Some(expanded)
        })
        .collect();

    let expanded = quote! {
        #(#push_to_collection_rules)*
    };

    expanded
}

pub(crate) fn pull_from_collection_methods(fields: &CharybdisFields) -> TokenStream {
    let pull_from_collection_rules: Vec<TokenStream> = fields
        .db_fields
        .iter()
        .filter_map(|field| {
            if !field.is_collection() {
                return None;
            }

            let pull_from_query_str = format!("Self::PULL_{}_QUERY", field.name.to_uppercase());
            let pull_from_query = parse_str::<TokenStream>(&pull_from_query_str).unwrap();
            let fun_name_str = format!("pull_{}", field.name);
            let fun_name = parse_str::<TokenStream>(&fun_name_str).unwrap();
            let types = fields.primary_key_fields.types();
            let values = fields.primary_key_fields.values();

            let expanded = quote! {
                pub fn #fun_name<V: charybdis::scylla::SerializeValue>(
                    &self,
                    value: V
                ) -> charybdis::query::CharybdisQuery<(V, #(#types),*), Self, charybdis::query::ModelMutation> {
                    charybdis::query::CharybdisQuery::new(
                        #pull_from_query,
                        charybdis::query::QueryValue::Owned((value, #(#values),*)),
                    )
                }
            };

            Some(expanded)
        })
        .collect();

    let expanded = quote! {
        #(#pull_from_collection_rules)*
    };

    expanded
}

pub(crate) fn pull_from_collection_methods_if_exists(fields: &CharybdisFields) -> TokenStream {
    let pull_from_collection_rules: Vec<TokenStream> = fields
        .db_fields
        .iter()
        .filter_map(|field| {
            if !field.is_collection() {
                return None;
            }

            let pull_from_query_str = format!("Self::PULL_{}_IF_EXISTS_QUERY", field.name.to_uppercase());
            let pull_from_query = parse_str::<TokenStream>(&pull_from_query_str).unwrap();
            let fun_name_str = format!("pull_{}_if_exists", field.name);
            let fun_name = parse_str::<TokenStream>(&fun_name_str).unwrap();
            let types = fields.primary_key_fields.types();
            let values = fields.primary_key_fields.values();

            let expanded = quote! {
                pub fn #fun_name<V: charybdis::scylla::SerializeValue>(
                    &self,
                    value: V
                ) -> charybdis::query::CharybdisQuery<(V, #(#types),*), Self, charybdis::query::ModelMutation> {
                    charybdis::query::CharybdisQuery::new(
                        #pull_from_query,
                        charybdis::query::QueryValue::Owned((value, #(#values),*)),
                    )
                }
            };

            Some(expanded)
        })
        .collect();

    let expanded = quote! {
        #(#pull_from_collection_rules)*
    };

    expanded
}
