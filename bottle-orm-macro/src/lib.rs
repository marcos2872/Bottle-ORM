use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, GenericArgument, PathArguments, Type};

fn rust_type_to_sql(ty: &Type) -> (String, bool) {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();

            if type_name == "Option" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        let (inner_sql_type, _ignored_nullable) = rust_type_to_sql(inner_ty);
                        return (inner_sql_type, true);
                    }
                }
            }

            match type_name.as_str() {
                "i32" => ("INTEGER".to_string(), false),
                "i64" => ("BIGINT".to_string(), false),
                "String" => ("TEXT".to_string(), false),
                "bool" => ("BOOLEAN".to_string(), false),
                "f64" => ("DOUBLE PRECISION".to_string(), false),
                "DateTime" => ("TIMESTAMPTZ".to_string(), false),
                "NaiveDateTime" => ("TIMESTAMP".to_string(), false),
                "NaiveDate" => ("DATE".to_string(), false),
                "NaiveTime" => ("TIME".to_string(), false),
                _ => ("TEXT".to_string(), false),
            }
        } else {
            ("TEXT".to_string(), false)
        }
    } else {
        ("TEXT".to_string(), false)
    }
}

#[proc_macro_derive(Model, attributes(orm))]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let fields = if let Data::Struct(data) = &ast.data {
        if let Fields::Named(fields) = &data.fields {
            fields
        } else {
            panic!("Model is empty");
        }
    } else {
        panic!("Model is not a struct")
    };

    let column_defs = fields.named.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        let (mut sql_type, is_nullable) = rust_type_to_sql(field_type);
        let mut is_primary_key = false;
        let mut size = None;
        let mut create_time = false;
        let mut update_time = false;
        let mut unique = false;
        let mut index = false;
        let mut foreign_table_tokens = quote! { None };
        let mut foreign_key_tokens = quote! { None };

        for attr in &f.attrs {
            if attr.path().is_ident("orm") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("primary_key") {
                        is_primary_key = true;
                    }

                    if meta.path.is_ident("size") {
                        let value: syn::LitInt = meta.value()?.parse()?;
                        size = Some(value.base10_parse::<usize>()?);
                    }

                    if meta.path.is_ident("create_time") {
                        create_time = true;
                    }

                    if meta.path.is_ident("create_time") {
                        update_time = true;
                    }

                    if meta.path.is_ident("unique") {
                        unique = true;
                    }

                    if meta.path.is_ident("index") {
                        index = true;
                    }

                    if meta.path.is_ident("foreign_key") {
                        let value: syn::LitStr = meta.value()?.parse()?;
                        let fk_string = value.value();

                        let parts: Vec<&str> = fk_string.split("::").collect();

                        if parts.len() == 2 {
                            let table = parts[0];
                            let col = parts[1];

                            foreign_table_tokens = quote! { Some(#table) };
                            foreign_key_tokens = quote! { Some(#col) };
                        } else {
                            return Err(meta.error("Invalid format for foreign_key. Use 'Table::column'"));
                        }
                    }

                    Ok(())
                })
                .unwrap();
            }
        }

        if let Some(s) = size {
            if sql_type == "TEXT" || sql_type == "VARCHAR" {
                sql_type = format!("VARCHAR({})", s);
            }
        }

        quote! {
            bottle_orm::ColumnInfo {
                 name: stringify!(#field_name),
                 sql_type: #sql_type,
                 is_primary_key: #is_primary_key,
                 is_nullable: #is_nullable,
                 create_time: #create_time,
                 update_time: #update_time,
                 unique: #unique,
                 index: #index,
                 foreign_table: #foreign_table_tokens,
                 foreign_key: #foreign_key_tokens
            }
        }
    });

    let field_names_iter: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
    let map_inserts = fields.named.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        let (_, is_nullable) = rust_type_to_sql(field_type);
        if is_nullable {
            return quote! {
                if let Some(val) = &self.#field_name {
                	map.insert(
                 	stringify!(#field_name).to_string(),
                 	val.to_string()
                );
             }
            };
        }

        quote! {
            map.insert(
                stringify!(#field_name).to_string(),
                 self.#field_name.to_string()
            );
        }
    });

    let expanded = quote! {
        impl bottle_orm::Model for #struct_name {
            fn table_name() -> &'static str {
                stringify!(#struct_name)
            }

            fn columns() -> Vec<bottle_orm::ColumnInfo> {
                vec![#(#column_defs),*]
            }

            fn active_columns() -> Vec<&'static str> {
                vec![#(stringify!(#field_names_iter) ),*]
            }

            fn to_map(&self) -> std::collections::HashMap<String, String> {
                let mut map = std::collections::HashMap::new();
                 #(#map_inserts)*
                  map
            }
        }
    };

    TokenStream::from(expanded)
}
