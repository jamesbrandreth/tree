use std::cell::RefCell;
use std::rc::Rc;

struct NodeData {
    data: &'static str,
    parent: Option<Node>,
    children: Vec<Node>,
}

#[derive(Clone)]
pub struct Node {
    data: Rc<RefCell<NodeData>>,
}

impl Node {
    pub fn new(data: &'static str) -> Self {
        let node_data = Rc::new(RefCell::new(NodeData {
            data,
            parent: None,
            children: vec![],
        }));

        Self { data: node_data }
    }

    pub fn parent(&self) -> Option<Node> {
        self.data.borrow().parent.clone()
    }

    fn set_parent(&mut self, parent: Node) {
        self.data.borrow_mut().parent = Some(parent);
    }

    pub fn add_child(&mut self, mut child: Node) {
        child.set_parent(self.clone());
        self.data.borrow_mut().children.push(child);
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let node_data = self.data.borrow();

        let mut children: String = node_data
            .children
            .iter()
            .map(|child| format!("{}\n", child))
            .collect();
        children = children
            .lines()
            .map(|line| format!("\n  {}", line))
            .collect();
        write!(f, "{}{}", node_data.data, format!("{}", children))
    }
}
