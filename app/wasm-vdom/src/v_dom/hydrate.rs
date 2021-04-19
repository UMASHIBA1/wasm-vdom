use crate::v_dom::types::{VirtualNode, DOMAttributes, ExpandElement, NodeType, KeyAttribute};
use crate::v_dom::types::NodeType::ElementNode;
use std::rc::Rc;

pub enum VNodeOrStr<'a> {
    VNode(Rc<VirtualNode>),
    StrText(&'a str)
}

pub fn create_text_vnode<'a>(text: &str, real_node: Option<ExpandElement>) -> Rc<VirtualNode> {
    VirtualNode::new(text, DOMAttributes::new(), Vec::new(), real_node, NodeType::TextNode, None)
}

pub fn hydrate(
    name: &str,
    props: DOMAttributes,
    children: Vec<VNodeOrStr>,
    real_node: Option<ExpandElement>
) -> Rc<VirtualNode> {
    let mut vnode_children: Vec<Rc<VirtualNode>> = Vec::new();
    for child in children {
        let vnode = match child {
            VNodeOrStr::VNode(vnode) => vnode,
            VNodeOrStr::StrText(text) => create_text_vnode(&text, None)
        };
        vnode_children.push(vnode);
    }

    let key: Option<KeyAttribute> = match props.get("key") {
      Some(key) => {
          Some(KeyAttribute::Str(key.clone()))
      },
        _ => None
    };

    VirtualNode::new(
        name, props, vnode_children, real_node, ElementNode, key
    )

}