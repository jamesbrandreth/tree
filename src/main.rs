mod node;
use node::Node;

use std::cell::RefCell;
use std::rc::Rc;

fn add_child(current_node: Rc<RefCell<Node>>, new_node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    current_node.borrow_mut().add_child(new_node.clone());
    new_node
}

fn main() {
    // Make a root node
    let root = Rc::new(RefCell::new(Node::new("Hi, i'm root!", None)));
    // Set the "current" node to the root
    let mut current_node = root.clone();

    // Make a child and add it to the root, update the current node to be the child
    let child = Rc::new(RefCell::new(Node::new(
        "Hi, i'm a child!",
        Some(root.clone()),
    )));
    current_node = add_child(current_node, child);

    let grandchild = Rc::new(RefCell::new(Node::new(
        "Hi, i'm a grandchild!",
        Some(current_node.clone()),
    )));
    current_node = add_child(current_node, grandchild);

    // Move up a node
    let next_node = current_node.borrow().parent().as_ref().unwrap().clone();
    current_node = next_node;

    let grandchild = Rc::new(RefCell::new(Node::new(
        "Hi, i'm another grandchild!",
        Some(current_node.clone()),
    )));
    current_node = add_child(current_node, grandchild);

    // Move up two nodes
    let next_node = current_node.borrow().parent().as_ref().unwrap().clone();
    current_node = next_node;
    let next_node = current_node.borrow().parent().as_ref().unwrap().clone();
    current_node = next_node;

    let grandchild = Rc::new(RefCell::new(Node::new(
        "Hi, i'm a child!",
        Some(current_node.clone()),
    )));
    current_node = add_child(current_node, grandchild);

    println!("{}", root.borrow());
}
