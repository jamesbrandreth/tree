use std::cell::RefCell;
use std::rc::Rc;

pub type SharedNode = Rc<RefCell<Node>>;

pub struct Node {
    data: &'static str,
    parent: Option<SharedNode>,
    children: Vec<SharedNode>,
}

impl Node {
    pub fn new(data: &'static str, parent: Option<SharedNode>) -> Self {
        Self {
            data,
            parent,
            children: vec![],
        }
    }

    pub fn parent(&self) -> &Option<SharedNode> {
        &self.parent
    }

    pub fn add_child(&mut self, child: SharedNode) {
        self.children.push(child);
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut children: String = self
            .children
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
