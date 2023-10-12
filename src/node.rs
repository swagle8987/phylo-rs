pub trait Node<T> {
    fn set_parent(&mut self, parent: usize);
    fn get_parent(&self)->Option<&usize>;
    fn get_parent_mut(&self)->Option<&mut usize>;
    fn get_child(&self, child:&T)->Option<&usize>;
    fn get_child_mut(&mut self, child:&T)->Option<&mut usize>;
    fn set_child(&mut self, edge:T, child:usize);
    fn get_edge_weight(&mut self, edge_label:T)-> Option<&isize>;
    fn get_edge_weight(&mut self, edge_label:T)-> Option<&mut isize>;
    fn set_edge_weight(&mut self, edge_label:T, edge_weight:isize);
    fn has_children(&self)->bool;
    fn get_children(&self)->&HashMap<T, (usize, isize)>;
}