use crate::model::Customer;
use crate::model::Order;
use crate::model::Product;
use crate::repo::CustomerRepo;
use crate::repo::OrderRepo;
use crate::repo::ProductRepo;
use crate::repo::Repositories;

pub struct SqliteCustomerRepo;
pub struct SqliteProductRepo;
pub struct SqliteOrderRepo;

pub struct SqliteRepositories;

impl CustomerRepo for SqliteCustomerRepo {
    fn login(&self, _customer: Customer) -> bool {
        println!("SqliteCustomerRepo::login");
        true
    }
    fn register(&self, _customer: Customer) -> bool {
        println!("SqliteCustomerRepo::register");
        true
    }
}

impl ProductRepo for SqliteProductRepo {
    fn find_by_id(&self, _id: u64) -> Option<Product> {
        println!("SqliteProductRepo::find_by_id");
        None
    }
    fn list_all(&self) -> Vec<Product> {
        println!("SqliteProductRepo::list_all");
        vec![]
    }
    fn create(&self, _product: Product) -> bool {
        println!("SqliteProductRepo::create");
        true
    }
}

impl OrderRepo for SqliteOrderRepo {
    fn find_by_id(&self, _id: u64) -> Option<Order> {
        println!("SqliteOrderRepo::find_by_id");
        None
    }
    fn list_by_customer(&self, _customer_id: u64) -> Vec<Order> {
        println!("SqliteOrderRepo::list_by_customer");
        vec![]
    }
    fn create(&self, _order: Order) -> bool {
        println!("SqliteOrderRepo::create");
        true
    }
}

impl Repositories for SqliteRepositories {
    fn customer(&self) -> Box<dyn CustomerRepo> {
        Box::new(SqliteCustomerRepo)
    }

    fn product(&self) -> Box<dyn ProductRepo> {
        Box::new(SqliteProductRepo)
    }

    fn order(&self) -> Box<dyn OrderRepo> {
        Box::new(SqliteOrderRepo)
    }
}
