mod node;
use node::Node;

fn main() {
    // Make a root node
    let root = Node::new("Hi, i'm root!");
    // Set the "current" node to the root
    let mut current_node = root.clone();

    // Make a child and add it to the root, update the current node to be the child
    let child = Node::new("Hi, i'm a child!");
    current_node.add_child(child.clone());
    current_node = child;

    let grandchild = Node::new("Hi, i'm a grandchild!");
    current_node.add_child(grandchild.clone());
    current_node = grandchild;

    // Move up a node
    let next_node = current_node.parent().unwrap();
    current_node = next_node;

    let grandchild = Node::new("Hi, i'm another grandchild!");
    current_node.add_child(grandchild.clone());
    current_node = grandchild;

    // Move up two nodes
    let next_node = current_node.parent().unwrap();
    current_node = next_node;
    let next_node = current_node.parent().unwrap();
    current_node = next_node;

    let grandchild = Node::new("Hi, i'm a child!");
    current_node.add_child(grandchild);

    println!("{}", root);
}
