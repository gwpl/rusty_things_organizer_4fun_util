use ascii_tree::*;
use ascii_tree::Tree::*;
use ansi_term::Colour::*;

fn tree_from_documentation() {
    let l1 = Leaf(vec![String::from("line1"), String::from("line2"), String::from("line3"), String::from("line4")]);
    let l2 = Leaf(vec![String::from("only one line")]);
    let n1 = Node(String::from("node 1"), vec![l1.clone(), l2.clone()]);
    let n2 = Node(String::from("node 2"), vec![l2.clone(), l1.clone(), l2.clone()]);
    let n3 = Node(String::from("node 3"), vec![n1.clone(), l1.clone(), l2.clone()]);
    let n4 = Node(String::from("node 4"), vec![n1, n2, n3]);
     
    let mut output = String::new();
    let _ = write_tree(&mut output, &n4);
    println!("{}", output);
}

fn tree_with_asciiterm() {
    let l1 = Leaf(vec![Blue.paint("line1").to_string(), Red.paint("line2").to_string(), Purple.strikethrough().paint("line3").to_string(), Cyan.paint("line4").to_string()]);
    let l2 = Leaf(vec![White.blink().paint("only one line").to_string()]);
    let n1 = Node(format!("{} 1 {}", Green.paint("node"),  Green.bold().paint("(green comment)")), vec![l1.clone(), l2.clone()]);
    let n2 = Node(White.bold().paint("node 2").to_string(), vec![l2.clone(), l1.clone(), l2.clone()]);
    let n3 = Node(Yellow.underline().paint("node 3").to_string(), vec![n1.clone(), l1.clone(), l2.clone()]);
    let n4 = Node(String::from("node 4"), vec![n1, n2, n3]);
     
    let mut output = String::new();
    let _ = write_tree(&mut output, &n4);
    println!("{}", output);
}

fn main() {
    println!("{}", Green.paint("Tree from documentation:"));
    tree_from_documentation();
    println!("{}", Blue.paint("Tree with asciiterm:"));
    tree_with_asciiterm();
}
