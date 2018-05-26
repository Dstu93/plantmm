

#[derive(Ord, PartialOrd, Eq, PartialEq,Clone,Debug)]
pub struct DependencyTree<'a>{
    root: Node<'a>,
}

impl  <'a>DependencyTree<'a>{

    //Creates a new DependencyTree with the given Node as root
    pub fn new(root: Node<'a>) -> DependencyTree {
        DependencyTree{root}
    }

}


#[derive(Ord, PartialOrd, Eq, PartialEq,Clone,Debug)]
pub struct Node<'a>{
    name: &'a str,
    shape: Shape,
    children: Option<Vec<Node<'a>>>,
}

impl <'a>Node<'a>{

    pub fn new(name: &'a str, shape: Shape,children: Option<Vec<Node<'a>>>) -> Node<'a>{
        Node{name,shape,children}
    }

    pub fn name(&self) -> &str{
        self.name
    }

}

#[derive(Ord, PartialOrd, Eq, PartialEq,Clone,Debug,Hash)]
pub enum Shape{
    Circle,
    Rect,
    Ellipse,
}