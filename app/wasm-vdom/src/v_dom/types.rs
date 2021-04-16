use std::any::Any;
use std::collections::HashMap;
use web_sys::{HtmlElement, Text};
use js_sys::Function;

pub const TEXT_NODE: i64 = 3;

#[derive(Copy)]
pub enum KeyAttribute{
    str,
    i64
}

pub type DOMAttributes = HashMap<str, Any>;

pub type Handlers = HashMap<str, Function>;

pub struct ElementAndNeedAttr {
    element: HtmlElement,
    vdom: Option<VirtualNode>,
    event_handlers: Option<Handlers>
}

pub struct TextAndVdom {
    text: Text,
    vdom: Option<VirtualNode>,
}

pub enum ExpandElement {
    ElementAndNeedAttr,
    TextAndVdom
}

pub enum NodeType {
    TEXT_NODE,
    ELEMENT_NODE
}

pub struct VirtualNode {
    name: String,
    props: DOMAttributes,
    children: Vec<VirtualNode>,
    real_node: Option<ExpandElement>,
    node_type: NodeType,
    key: Option<KeyAttribute>
}

impl VirtualNode {
    pub fn new(
        name: impl Into<String>,
        props: DOMAttributes,
        children: Vec<VirtualNode>,
        real_node: Option<ExpandElement>,
        node_type: NodeType,
        key: Option<KeyAttribute>
    ) -> VirtualNode {
        VirtualNode {
            name,
            props,
            children,
            real_node,
            node_type,
            key
        }
    }
}
