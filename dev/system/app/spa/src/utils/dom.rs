//------------------------------------------------------------------------------
//! DOM utility.
//------------------------------------------------------------------------------

use sycamore::prelude::*;

pub fn get_child_nodes( dom_node: &DomNode ) -> Vec<DomNode>
{
    let node = dom_node.to_web_sys();
    let child_nodes = node.child_nodes();
    let mut children = Vec::new();
    for i in 0..child_nodes.length()
    {
        let child = child_nodes.get(i).unwrap();
        children.push(DomNode::from_web_sys(child));
    }
    children
}
