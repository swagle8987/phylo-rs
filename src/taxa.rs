pub type TaxaID = usize;

pub trait Taxa<T>{
    fn get_id(&self)->T;
}