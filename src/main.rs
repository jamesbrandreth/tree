use std::cell::RefCell;
use std::rc::Rc;


struct Node {
    data: &'static str,
    pub parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.push(child);
    }

}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut children: String = self.children
            .iter()
            .map(|child| format!("{}\n", child.borrow()))
            .collect();
        children = children
            .lines()
            .map(|line| format!("\n  {}", line))
            .collect();
        write!(f, "{}{}", self.data, format!("{}", children))
    }
}

fn add_child(current_node: Rc<RefCell<Node>>, new_node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>>{
    current_node.borrow_mut().add_child(new_node.clone());
    new_node
}

fn main() {
    let root = Rc::new(RefCell::new(Node{
        data: "Hi, i'm root!",
        parent: None,
        children: vec![],
    }));
    let mut current_node = root.clone();

    let child = Rc::new(RefCell::new(Node{
        data: "Hi, i'm a child!",
        parent: Some(root.clone()),
        children: vec![],
    }));
    current_node = add_child(current_node, child);

    let grandchild = Rc::new(RefCell::new(Node{
        data: "Hi, i'm a grandchild!",
        parent: Some(current_node.clone()),
        children: vec![],
    }));
    current_node = add_child(current_node, grandchild);
    let next_node = current_node.borrow().parent.as_ref().unwrap().clone();
    current_node = next_node;

    let grandchild = Rc::new(RefCell::new(Node{
        data: "Hi, i'm another grandchild!",
        parent: Some(current_node.clone()),
        children: vec![],
    }));
    current_node = add_child(current_node, grandchild);
    let next_node = current_node.borrow().parent.as_ref().unwrap().clone();
    current_node = next_node;
    let next_node = current_node.borrow().parent.as_ref().unwrap().clone();
    current_node = next_node;
    let grandchild = Rc::new(RefCell::new(Node{
        data: "Hi, i'm a child!",
        parent: Some(current_node.clone()),
        children: vec![],
    }));
    current_node = add_child(current_node, grandchild);
    println!("{}", root.borrow());
}
