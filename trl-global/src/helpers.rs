use syn::{Field, Fields, Visibility};

use crate::Attrs;

pub fn fill_includes_if_empty(includes: &mut Vec<String>, fields: &Fields) {
    if !includes.is_empty() {
        return;
    }

    for field in fields.iter() {
        includes.push(field.ident.clone().unwrap().to_string());
    }
}

pub fn should_add_pub(attrs: &Attrs, field: &Field) -> bool {
    if !attrs.include_pub {
        if let Visibility::Public(..) = field.vis {
            return false;
        }
    };

    true
}

pub fn should_include(attrs: &Attrs, field: &Field) -> bool {
    if !attrs
        .includes
        .contains(&field.ident.clone().unwrap().to_string())
        || attrs
            .excludes
            .contains(&field.ident.clone().unwrap().to_string())
    {
        return false;
    }

    true
}

pub fn should_field_be_added(attrs: &Attrs, field: &Field) -> bool {
    should_add_pub(&attrs, &field) && should_include(&attrs, &field)
}
