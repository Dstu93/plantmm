
use std::fs::File;

use parser::DependencyTreeParser;
use model::{DependencyTree,Node,Shape};

// ** Content of the test file
//
// title = Countries
//
//@start
//
//Europe -- England
//Europe -- France
//Europe -- Spain
//
//England -- London
//England -- Oxford
//
//France -- Paris
//
//@end
//
#[test]
pub fn parse_file(){
    //testfile
    //parse
    //verify

    let mut textfile = File::open("src/tests/test_mindmap.txt").expect("file could not be found");
    let parsed_tree = DependencyTreeParser::parse_from_file(&mut textfile).expect("could not parse tree from file");

    //The correct nodes which should be parsed from the text file
    let london = Node::new("London",Shape::Ellipse,None);
    let oxford = Node::new("Oxford",Shape::Ellipse,None);
    let eng = Node::new("England",Shape::Ellipse,Some(vec!{london,oxford}));
    let paris  =  Node::new("Paris",Shape::Ellipse,None);
    let spain = Node::new("Spain",Shape::Ellipse,None);
    let fr = Node::new("France",Shape::Ellipse, Some(vec!{paris}));

    let eur = Node::new("Europe",Shape::Ellipse,Some(vec!{eng,fr,spain}));

    let correct_tree = DependencyTree::new(eur);

    assert_eq!(parsed_tree,correct_tree);
}