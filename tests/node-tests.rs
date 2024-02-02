
#[cfg(test)]
mod tests {

    use std::rc::Rc;
    use phylo::node::Node;
    fn test_flip(){
	let mut n = Node::new(Rc::new(5),true);
	assert_eq!(n.is_leaf(),true);
	n.flip();
	assert_eq!(n.is_leaf(),false);
	let mut a = Node::new(0,false);
	assert_eq!(n.is_leaf(),false);
	a.flip();
	assert_eq!(n.is_leaf(),true);
    }


    fn test_set_id(){
	let mut n = Node::new(0,true);
	assert_eq!(n.get_id(),0);
	n.set_id(10);
	assert_eq!(n.get_id(),10);
    }


    fn test_set_taxa(){
	let mut n = Node::new(0,true);
	assert_eq!(n.get_taxa(),None);
	n.set_taxa = Some(String::from("A"));
	assert_eq!(n.set_taxa()?,String::from("A"));
    }

    fn test_parent_childs(){
	let mut n = Node::new(0,true);
	n.add_child(10);
	n.add_child(20);
	assert_eq!(n.get_children(),[10,20]);
	n.remove_child(10);
	assert_eq!(n.get_children(),[20]);
	n.set_parent(Some(10));
	assert_eq!(n.get_parent()?,10);
	
    }
}
