use std::{collections::HashMap, default, ops::{DerefMut, Deref}, fmt::Display};

use fancy_regex::{Regex, CaptureMatches};
use html_parser::{Dom, Node, Element};
///html gets put in a tree like structure



#[derive(Debug, Clone)]
pub struct HtmlRoot{
    root: Dom
}
impl HtmlRoot{
    //convert to custom tree to allow for greater flexibility
    pub fn from_str(val: &str) -> Self {
        let node = Dom::parse(val).unwrap();
        Self {
            root: node
        }
    }
    pub fn replace(&mut self, old: &str, new: &str){
        let old = format!("|{}|", old);
        self.root.children.iter_mut()
            .for_each(|item| {
                match item {
                    Node::Element(x) => Self::replace_rec(x, &old, new),
                    Node::Text(str) => *str = str.replace(&old, new),
                    _ => ()
                };
            })
    }
    pub fn replace_component(&mut self, old: &str, new: &str){
        self.root.children.iter_mut()
            .for_each(|item| {
                match item {
                    Node::Element(x) => {
                        let newnode = Node::Text(new.to_string());
                        if x.name == old {
                            x.children = vec![newnode];
                        } else {
                            Self::replace_component_rec(x, old, new);
                        }
                    },
                    _ => ()
                };
            });
    }
    fn replace_component_rec(node: &mut Element, old: &str, new: &str){
        node.children.iter_mut()
            .for_each(|item| {
                match item {
                    Node::Element(x) => {
                        let newnode = Node::Text(new.to_string());
                        if x.name == old {
                            x.children = vec![newnode];
                        } else {
                            Self::replace_component_rec(x, old, new);
                        }
                    },
                    _ => ()
                };
        });
    }
    fn replace_rec(node: &mut Element, old: &str, new: &str){
        node.children.iter_mut()
            .for_each(|item| {
                match item {
                    Node::Element(x) => Self::replace_rec(x, old, new),
                    Node::Text(str) => *str = str.replace(old, new),
                    _ => ()
                };
            })
    }


}

impl Display for HtmlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = self.root.children.iter()
        .map(Self::node_display).collect();
        write!(f, "{}", str)
    }
}
impl HtmlRoot {
    fn element_display(ell: &Element) -> String {
        ell.children.iter()
            .map(Self::node_display).collect()
    }
    fn node_display(item: &Node) -> String {
        match item {
            Node::Text(x) => x.to_owned(),
            Node::Element(x) => {
                let name = x.name.as_str();
                let value = Self::element_display(x);
                let mut attrs = x.id.as_ref().map_or("".to_string(), |x| format!(r#" id="{x}""#));
                let attr: String = x.attributes.iter().map(|(a,b)| format!(r#" {}="{}""#, a, b.as_ref().unwrap_or(&"".to_string()))).collect();
                if !x.classes.is_empty(){
                    attrs += &format!(r#" class="{}""#, x.classes.join(" "));
                }
                attrs += &format!("{}", attr);
                format!("<{name}{attrs}>{value}</{name}>")
            },
            _ => "".to_owned()
        }
    }
}

#[cfg(test)]
mod tests{
    use crate::html;

    use super::HtmlRoot;
    use html_parser::Dom;

    #[test]
    fn simple_parse(){
        let mut node = HtmlRoot::from_str(r#"<div class="wrapper" id="main"><user /></div>"#);
        node.replace("name", "test");
        println!("{node:?}");
    }
}