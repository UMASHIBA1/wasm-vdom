use std::collections::HashMap;
use web_sys::{Text, Element};
use js_sys::Function;
use std::rc::Rc;

pub const TEXT_NODE: u16 = 3;

#[derive(Clone)]
pub enum KeyAttribute{
    Str(String),
    Num(i64)
}

pub type DOMAttributes = HashMap<String, String>;

pub type Handlers = HashMap<String, Function>;

pub struct ElementAndNeedAttr {
    pub element: Element,
    pub vdom: Option<Rc<VirtualNode>>,
    pub event_handlers: Option<Handlers>
}

impl ElementAndNeedAttr {
    pub fn new(element: Element, vdom: Option<Rc<VirtualNode>>, event_handlers: Option<Handlers>) -> ElementAndNeedAttr {
        ElementAndNeedAttr {element, vdom, event_handlers}
    }
}

pub struct TextAndVdom {
    text: Text,
    vdom: Option<Rc<VirtualNode>>,
}

impl TextAndVdom {
    pub fn new(text: Text, vdom: Option<Rc<VirtualNode>>) -> TextAndVdom {
        TextAndVdom {text, vdom}
    }
}

// NOTE: Element系がヒープとして保存されてるか不安なので後で確認する
pub enum ExpandElement {
    ElementAndNeedAttr(ElementAndNeedAttr),
    TextAndVdom(TextAndVdom)
}

pub enum NodeType {
    TextNode,
    ElementNode
}

pub struct VirtualNode {
    name: String,
    props: DOMAttributes,
    children: Vec<Rc<VirtualNode>>,
    real_node: Option<ExpandElement>,
    node_type: NodeType,
    key: Option<KeyAttribute>
}

impl VirtualNode {
    pub fn new(
        name: impl Into<String>,
        props: DOMAttributes,
        children: Vec<Rc<VirtualNode>>,
        real_node: Option<ExpandElement>,
        node_type: NodeType,
        key: Option<KeyAttribute>
    ) -> Rc<VirtualNode> {
        Rc::new(VirtualNode {
            name: name.into(),
            props,
            children,
            real_node,
            node_type,
            key
        })
    }
}
