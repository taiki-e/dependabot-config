#![warn(rust_2018_idioms, single_use_lifetimes)]

#[macro_use]
mod file;

use std::{collections::BTreeSet, path::Path};

use anyhow::Result;
use fs_err as fs;
use heck::{ToKebabCase, ToSnakeCase};
use quote::{format_ident, quote};
use syn::visit_mut::{self, VisitMut};

use crate::file::*;

fn main() -> Result<()> {
    gen_assert_impl()?;
    gen_display()?;
    gen_from_str()?;
    Ok(())
}

fn gen_from_str() -> Result<()> {
    let workspace_root = &workspace_root();

    let mut tokens = quote! {
        use core::str::FromStr;
        use crate::Error;
    };

    let files = &["src/lib.rs", "src/v1.rs", "src/v2.rs"];

    for &f in files {
        let s = fs::read_to_string(workspace_root.join(f))?;
        let mut ast = syn::parse_file(&s)?;

        let module = if f.ends_with("lib.rs") {
            vec![]
        } else {
            let name = format_ident!("{}", Path::new(f).file_stem().unwrap().to_string_lossy());
            vec![name.into()]
        };

        ItemVisitor::new(module, |item, module| match item {
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
        })
        .visit_file_mut(&mut ast);
    }

    write(function_name!(), &workspace_root.join("src/gen/from_str.rs"), tokens)?;

    Ok(())
}

fn serde_attr(attrs: &[syn::Attribute], name: &str) -> Option<String> {
    for meta in attrs
        .iter()
        .filter(|attr| attr.path.is_ident("serde"))
        .filter_map(|attr| attr.parse_meta().ok())
    {
        if let syn::Meta::List(list) = meta {
            for repr in list.nested {
                if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = repr {
                    if nv.path.is_ident(name) {
                        if let syn::Lit::Str(s) = nv.lit {
                            return Some(s.value());
                        }
                    }
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

fn gen_display() -> Result<()> {
    let workspace_root = &workspace_root();

    let mut tokens = quote! {
        use core::fmt;
    };

    let files = &["src/v1.rs", "src/v2.rs"];

    for &f in files {
        let s = fs::read_to_string(workspace_root.join(f))?;
        let mut ast = syn::parse_file(&s)?;

        let module = {
            let name = format_ident!("{}", Path::new(f).file_stem().unwrap().to_string_lossy());
            vec![name.into()]
        };

        ItemVisitor::new(module, |item, module| match item {
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
        })
        .visit_file_mut(&mut ast);
    }

    write(function_name!(), &workspace_root.join("src/gen/display.rs"), tokens)?;

    Ok(())
}

fn gen_assert_impl() -> Result<()> {
    let workspace_root = &workspace_root();
    let out_dir = &workspace_root.join("src/gen");
    fs::create_dir_all(out_dir)?;

    let files: BTreeSet<String> = ignore::Walk::new(workspace_root.join("src"))
        .filter_map(Result::ok)
        .filter_map(|e| {
            let path = e.path();
            if !path.is_file() || path.extension() != Some("rs".as_ref()) {
                return None;
            }
            // Assertions are only needed for the library's public APIs.
            if path.ends_with("main.rs") {
                return None;
            }
            Some(path.to_string_lossy().into_owned())
        })
        .collect();

    let mut tokens = quote! {};
    for f in &files {
        let s = fs::read_to_string(f)?;
        let mut ast = syn::parse_file(&s)?;

        let module = if f.ends_with("lib.rs") {
            vec![]
        } else {
            let name = format_ident!("{}", Path::new(f).file_stem().unwrap().to_string_lossy());
            vec![name.into()]
        };

        ItemVisitor::new(module, |item, module| match item {
            syn::Item::Struct(syn::ItemStruct { vis, ident, generics, .. })
            | syn::Item::Enum(syn::ItemEnum { vis, ident, generics, .. })
            | syn::Item::Union(syn::ItemUnion { vis, ident, generics, .. })
            | syn::Item::Type(syn::ItemType { vis, ident, generics, .. })
                if matches!(vis, syn::Visibility::Public(..)) =>
            {
                let lt_count = generics.lifetimes().count();
                let lt = if lt_count > 0 {
                    let lt = (0..lt_count).map(|_| quote! { '_ });
                    quote! { <#(#lt),*> }
                } else {
                    quote! {}
                };
                tokens.extend(quote! {
                    assert_auto_traits::<crate:: #(#module::)* #ident #lt>();
                });
            }
            _ => {}
        })
        .visit_file_mut(&mut ast);
    }

    let out = quote! {
        const _: fn() = || {
            fn assert_auto_traits<T: ?Sized + Send + Sync + Unpin>() {}
            #tokens
        };
    };
    write(function_name!(), &out_dir.join("assert_impl.rs"), out)?;

    Ok(())
}

struct ItemVisitor<F> {
    module: Vec<syn::PathSegment>,
    f: F,
}

impl<F> ItemVisitor<F>
where
    F: FnMut(&mut syn::Item, &[syn::PathSegment]),
{
    fn new(module: Vec<syn::PathSegment>, f: F) -> Self {
        Self { module, f }
    }
}

impl<F> VisitMut for ItemVisitor<F>
where
    F: FnMut(&mut syn::Item, &[syn::PathSegment]),
{
    fn visit_item_mut(&mut self, item: &mut syn::Item) {
        if let syn::Item::Mod(item) = item {
            self.module.push(item.ident.clone().into());
            visit_mut::visit_item_mod_mut(self, item);
            self.module.pop();
            return;
        }
        (self.f)(item, &self.module);
        visit_mut::visit_item_mut(self, item);
    }
}
