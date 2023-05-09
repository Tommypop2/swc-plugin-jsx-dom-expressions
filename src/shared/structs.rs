use std::collections::{HashMap, HashSet};
use swc_core::{common::comments::Comments, ecma::{ast::*, utils::private_ident}};

use crate::config::Config;

pub struct TemplateConstruction {
    pub template: String,
    pub id: Ident,
    pub is_svg: bool,
    pub is_ce: bool
}

#[derive(Clone, Debug)]
pub struct DynamicAttr {
    pub elem: Ident,
    pub key: String,
    pub value: Expr,
    pub is_svg: bool,
    pub is_ce: bool,
    pub tag_name: String
}

#[derive(Debug,Default)]
pub struct TemplateInstantiation {
    pub component: bool,
    pub template: String,
    pub declarations: Vec<VarDeclarator>,
    pub id: Option<Ident>,
    pub tag_name: String,
    pub exprs: Vec<Expr>,
    pub dynamics: Vec<DynamicAttr>,
    pub post_exprs: Vec<Expr>,
    pub is_svg: bool,
    pub is_void: bool,
    pub has_custom_element: bool,
    pub text: bool,
    pub dynamic: bool,
    pub to_be_closed: HashSet<String>,
    pub skip_template: bool
}

pub struct TransformVisitor<C>
where
    C: Comments,
{
    pub config: Config,
    pub template: Option<TemplateInstantiation>,
    pub templates: Vec<TemplateConstruction>,
    pub imports: HashMap<String, Ident>,
    pub comments: C,
    uid_identifier_map: HashMap<String, usize>
}

impl<C> TransformVisitor<C>
where
    C: Comments,
{
    pub fn new(config: Config, comments: C) -> Self {
        Self {
            config,
            templates: vec![],
            template: None,
            imports: HashMap::new(),
            comments,
            uid_identifier_map: HashMap::new()
        }
    }

    pub fn generate_uid_identifier(&mut self, name: &str) -> Ident {
        let name = if name.starts_with("_") {
            name.to_string()
        } else {
            "_".to_string() + name
        };
        if let Some(count) = self.uid_identifier_map.get_mut(&name) {
            *count = *count + 1;
            return private_ident!(format!("{name}{count}"));
        } else {
            self.uid_identifier_map.insert(name.clone(), 1);
            return private_ident!(name);
        }
    }
}

pub struct ProcessSpreadsInfo {
    pub elem: Option<Ident>,
    pub is_svg: bool,
    pub has_children: bool,
    pub wrap_conditionals: bool
}