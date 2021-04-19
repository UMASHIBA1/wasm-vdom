use crate::v_dom::types::{VirtualNode, ElementAndNeedAttr, TEXT_NODE, ExpandElement, DOMAttributes, NodeType};
use web_sys::{Node, Element, NamedNodeMap, Attr};
use crate::v_dom::hydrate::create_text_vnode;
use std::rc::Rc;

fn create_vnode_from_real_element(
    real_element: Element
) -> Rc<VirtualNode> {
    if real_element.node_type() == TEXT_NODE {
        let node_name: &String = &real_element.node_name();
        let real_element_and_need_attr = ExpandElement::ElementAndNeedAttr(ElementAndNeedAttr::new(real_element, None, None));
        create_text_vnode(node_name, Some(real_element_and_need_attr))
    } else {
        // NOTE: create children value
        let mut vnode_children: Vec<Rc<VirtualNode>> = Vec::new();
        let childrenLength: u32 = real_element.children().length();
        for i in 0..childrenLength {
            let child: Option<Element> = real_element.children().item(i);
            match child {
                Some(child) => {
                    let child_vnode = create_vnode_from_real_element(child);
                    vnode_children.push(child_vnode);
                },
                _ => {}
            }
        }

        // NOTE: create props value
        let mut props= DOMAttributes::new();
        if real_element.has_attributes() {
            let attrs: NamedNodeMap = real_element.attributes();
            let attrLength: u32 = attrs.length();
            for i in 0..attrLength {
                let attr_op: Option<Attr> = attrs.item(i);
                match attr_op {
                    Some(attr) => {
                        let attr_name: String = attr.name();
                        let attr_value: String = attr.value();
                        props.insert(attr_name, attr_value);
                    },
                    _ => {}
                }
            }
        }

        let node_name: &String = &real_element.node_name();
        let element_and_attr = ExpandElement::ElementAndNeedAttr(ElementAndNeedAttr::new(real_element, None, None));

        VirtualNode::new(node_name.to_lowercase(), props, vnode_children, Some(element_and_attr), NodeType::ElementNode, None)

    }
}

fn render(real_node: ElementAndNeedAttr, new_vnode: VirtualNode) {
    let element: Element = real_node.element;
    let parent_element: Option<Node> = element.parent_node();
    if parent_element.is_some() {
        let mut old_vnode: Option<VirtualNode>;

    }
}
