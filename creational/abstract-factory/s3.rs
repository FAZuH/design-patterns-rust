use crate::model::Customer;
use crate::model::Order;
use crate::model::Product;
use crate::repo::CustomerRepo;
use crate::repo::OrderRepo;
use crate::repo::ProductRepo;
use crate::repo::Repositories;

pub struct S3CustomerRepo;
pub struct S3ProductRepo;
pub struct S3OrderRepo;

pub struct S3Repositories;

impl CustomerRepo for S3CustomerRepo {
    fn login(&self, _customer: Customer) -> bool {
        println!("S3CustomerRepo::login");
        false
    }
    fn register(&self, _customer: Customer) -> bool {
        println!("S3CustomerRepo::register");
        false
    }
}

impl ProductRepo for S3ProductRepo {
    fn find_by_id(&self, _id: u64) -> Option<Product> {
        println!("S3ProductRepo::find_by_id");
        None
    }
    fn list_all(&self) -> Vec<Product> {
        println!("S3ProductRepo::list_all");
        vec![]
    }
    fn create(&self, _product: Product) -> bool {
        println!("S3ProductRepo::create");
        false
    }
}

impl OrderRepo for S3OrderRepo {
    fn find_by_id(&self, _id: u64) -> Option<Order> {
        println!("S3OrderRepo::find_by_id");
        None
    }
    fn list_by_customer(&self, _customer_id: u64) -> Vec<Order> {
        println!("S3OrderRepo::list_by_customer");
        vec![]
    }
    fn create(&self, _order: Order) -> bool {
        println!("S3OrderRepo::create");
        false
    }
}

impl Repositories for S3Repositories {
    fn customer(&self) -> Box<dyn CustomerRepo> {
        Box::new(S3CustomerRepo)
    }

    fn product(&self) -> Box<dyn ProductRepo> {
        Box::new(S3ProductRepo)
    }

    fn order(&self) -> Box<dyn OrderRepo> {
        Box::new(S3OrderRepo)
    }
}
