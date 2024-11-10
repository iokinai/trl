use syn::{punctuated::Punctuated, Expr, ExprArray, ExprPath, Meta, Token};

#[derive(Debug)]
pub enum Modifier {
    Borrow,
    Ref,
    MutRef,
}

#[derive(Debug)]
pub struct Attrs {
    pub includes: Vec<String>,
    pub excludes: Vec<String>,
    pub prefix: String,
    pub modifier: Modifier,
    pub include_pub: bool,
}

impl Attrs {
    fn path_to_string(expr: &Expr) -> String {
        if let Expr::Path(ExprPath { path, .. }) = expr {
            return path.get_ident().unwrap().to_string();
        };

        String::new()
    }

    fn parse_expr_array(value: &Expr, arr: &mut Vec<String>) {
        if let Expr::Array(ExprArray { elems, .. }) = value {
            elems.iter().for_each(|elem| {
                arr.push(Attrs::path_to_string(elem));
            });
        }
    }

    fn parse_expr_str(value: &Expr, str: &mut String) {
        *str = Attrs::path_to_string(value);
    }

    pub fn new(punctuated: Punctuated<Meta, Token![,]>) -> Attrs {
        let mut includes = Vec::new();
        let mut excludes = Vec::new();
        let mut prefix = String::new();
        let mut modifier = Modifier::Ref;
        let mut include_pub = false;
        for value in punctuated {
            match value {
                Meta::NameValue(nv) => match nv.path.get_ident().unwrap().to_string().as_str() {
                    "includes" => Attrs::parse_expr_array(&nv.value, &mut includes),
                    "excludes" => Attrs::parse_expr_array(&nv.value, &mut excludes),
                    "prefix" => Attrs::parse_expr_str(&nv.value, &mut prefix),
                    _ => {}
                },
                Meta::Path(p) => match p.get_ident().unwrap().to_string().as_str() {
                    "borrow" => modifier = Modifier::Borrow,
                    "mut_ref" => modifier = Modifier::MutRef,
                    "include_pub" => include_pub = true,
                    _ => {}
                },
                _ => {}
            };
        }

        Attrs {
            includes,
            excludes,
            prefix,
            modifier,
            include_pub,
        }
    }
}
