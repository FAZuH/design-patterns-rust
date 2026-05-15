use crate::model::Customer;
use crate::model::Order;
use crate::model::Product;

// The "Product"s
pub trait CustomerRepo {
    fn login(&self, customer: Customer) -> bool;
    fn register(&self, customer: Customer) -> bool;
}

pub trait ProductRepo {
    fn find_by_id(&self, id: u64) -> Option<Product>;
    fn list_all(&self) -> Vec<Product>;
    fn create(&self, product: Product) -> bool;
}

pub trait OrderRepo {
    fn find_by_id(&self, id: u64) -> Option<Order>;
    fn list_by_customer(&self, customer_id: u64) -> Vec<Order>;
    fn create(&self, order: Order) -> bool;
}

// The "Factory"
pub trait Repositories {
    fn customer(&self) -> Box<dyn CustomerRepo>;
    fn product(&self) -> Box<dyn ProductRepo>;
    fn order(&self) -> Box<dyn OrderRepo>;
}
