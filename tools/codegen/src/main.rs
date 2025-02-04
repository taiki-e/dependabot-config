// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(clippy::needless_pass_by_value, clippy::wildcard_imports)]

#[macro_use]
mod file;

use std::path::Path;

use fs_err as fs;
use heck::{ToKebabCase as _, ToSnakeCase as _};
use quote::{format_ident, quote};
use syn::{punctuated::Punctuated, *};

use crate::file::*;

fn main() {
    gen_from_str();
    gen_display();
    gen_assert_impl();
    gen_track_size();
}

fn gen_from_str() {
    let workspace_root = &workspace_root();

    let mut tokens = quote! {
        use core::str::FromStr;
        use crate::Error;
    };

    let files = &["src/lib.rs", "src/v1.rs", "src/v2.rs"];

    for &f in files {
        let s = fs::read_to_string(workspace_root.join(f)).unwrap();
        let ast = syn::parse_file(&s).unwrap();

        let module = if f.ends_with("lib.rs") {
            vec![]
        } else {
            let name = format_ident!("{}", Path::new(f).file_stem().unwrap().to_string_lossy());
            vec![name.into()]
        };

        test_helper::codegen::visit_items(module, ast, |item, module| match item {
            syn::Item::Struct(syn::ItemStruct { vis, ident, .. })
            | syn::Item::Enum(syn::ItemEnum { vis, ident, .. })
                if matches!(vis, syn::Visibility::Public(..)) =>
            {
                tokens.extend(quote! {
                    impl FromStr for crate:: #(#module::)* #ident {
                        type Err = Error;
                        fn from_str(s: &str) -> Result<Self, Self::Err> {
                            serde_yaml::from_str(s).map_err(Error::new)
                        }
                    }
                });
            }
            _ => {}
        });
    }

    write(function_name!(), workspace_root.join("src/gen/from_str.rs"), tokens).unwrap();
}

fn serde_attr(attrs: &[syn::Attribute], name: &str) -> Option<String> {
    for meta in attrs
        .iter()
        .filter(|attr| attr.path().is_ident("serde"))
        .filter_map(|attr| {
            attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated).ok()
        })
        .flatten()
    {
        if let syn::Meta::NameValue(nv) = meta {
            if nv.path.is_ident(name) {
                if let Expr::Lit(ExprLit { lit: syn::Lit::Str(s), .. }) = nv.value {
                    return Some(s.value());
                }
            }
        }
    }
    None
}

fn change_case(case: Option<&str>, value: String) -> String {
    match case {
        None => value,
        Some("kebab-case") => value.to_kebab_case(),
        Some("snake_case") => value.to_snake_case(),
        Some(case) => panic!("unknown case: {case}"),
    }
}

fn gen_display() {
    let workspace_root = &workspace_root();

    let mut tokens = quote! {
        use core::fmt;
    };

    let files = &["src/v1.rs", "src/v2.rs"];

    for &f in files {
        let s = fs::read_to_string(workspace_root.join(f)).unwrap();
        let ast = syn::parse_file(&s).unwrap();

        let module = {
            let name = format_ident!("{}", Path::new(f).file_stem().unwrap().to_string_lossy());
            vec![name.into()]
        };

        test_helper::codegen::visit_items(module, ast, |item, module| match item {
            syn::Item::Enum(syn::ItemEnum { attrs, vis, ident, variants, .. })
                if matches!(vis, syn::Visibility::Public(..))
                    && variants.iter().all(|v| matches!(v.fields, syn::Fields::Unit)) =>
            {
                let case = serde_attr(attrs, "rename_all");
                let arms = variants.iter().map(|syn::Variant { attrs, ident, .. }| {
                    let rename = serde_attr(attrs, "rename");
                    let s = if let Some(rename) = rename {
                        rename
                    } else {
                        change_case(case.as_deref(), ident.to_string())
                    };
                    quote! {
                        Self::#ident => f.write_str(#s),
                    }
                });
                tokens.extend(quote! {
                    impl fmt::Display for crate:: #(#module::)* #ident {
                        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                            match self {
                                #(#arms)*
                            }
                        }
                    }
                });
            }
            _ => {}
        });
    }

    write(function_name!(), workspace_root.join("src/gen/display.rs"), tokens).unwrap();
}

fn gen_assert_impl() {
    let (path, out) = test_helper::codegen::gen_assert_impl(
        &workspace_root(),
        test_helper::codegen::AssertImplConfig {
            exclude: &[],
            not_send: &[],
            not_sync: &[],
            not_unpin: &[],
            not_unwind_safe: &["error::Error"],
            not_ref_unwind_safe: &["error::Error"],
        },
    );
    write(function_name!(), path, out).unwrap();
}

fn gen_track_size() {
    let (path, out) = test_helper::codegen::gen_track_size(
        &workspace_root(),
        test_helper::codegen::TrackSizeConfig { exclude: &[] },
    );
    write(function_name!(), path, out).unwrap();
}
