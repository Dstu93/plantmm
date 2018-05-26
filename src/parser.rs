
use model::{Node,DependencyTree,Shape};

use std::io::Error;
use std::io::Read;
use std::fs::File;

pub struct DependencyTreeParser;

impl DependencyTreeParser{

    pub fn parse_from_file(file: &mut File) -> Result<DependencyTree,Error>{
        let mut content = String::new();
        //read from file
        file.read_to_string(&mut content)?;

        Ok(DependencyTree::new(Node::new("root",Shape::Ellipse,None)))
    }

}

