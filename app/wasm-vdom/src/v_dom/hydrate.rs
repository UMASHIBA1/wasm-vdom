use crate::v_dom::types::{VirtualNode, DOMAttributes, ExpandElement, NodeType, KeyAttribute};
use crate::v_dom::types::NodeType::ELEMENT_NODE;

enum VNodeOrStr {
    VirtualNode,
    str
}


fn create_text_vnode(text: &str, real_node: Option<ExpandElement>) -> VirtualNode {
    VirtualNode::new(text, DOMAttributes::new(), Vec::new(), real_node, NodeType::TEXT_NODE, None)
}

pub fn hydrate(
    name: &str,
    props: DOMAttributes,
    children: Vec<VNodeOrStr>,
    real_node: Option<ExpandElement>
) -> VirtualNode {
    let mut vnode_children: Vec<VirtualNode> = Vec::new();
    for child in children {
        let vnode = match child {
            VirtualNode(vnode) => vnode,
            str(text) => create_text_vnode(text, None)
        };
        vnode_children.push(vnode);
    }

    let key: Option<KeyAttribute> = match props.get("key") {
      Some(will_key) => if let Some(key) =  will_key.downcast_ref::<KeyAttribute>() {
          Some(**key)
      }else {None},
        _ => None
    };

    VirtualNode::new(
        name, props,vnode_children,real_node, ELEMENT_NODE, key
    )

}