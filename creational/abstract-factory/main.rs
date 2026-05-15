pub mod model;
pub mod repo;
pub mod s3;
pub mod sqlite;

use crate::model::Customer;
use crate::model::Order;
use crate::model::Product;
use crate::repo::Repositories;
use crate::s3::S3Repositories;
use crate::sqlite::SqliteRepositories;

fn main() {
    let sqlite = SqliteRepositories;

    sqlite.customer().login(Customer);
    sqlite.customer().register(Customer);

    sqlite.product().find_by_id(1);
    sqlite.product().list_all();
    sqlite.product().create(Product);

    sqlite.order().find_by_id(1);
    sqlite.order().list_by_customer(1);
    sqlite.order().create(Order);

    let s3 = S3Repositories;

    s3.customer().login(Customer);
    s3.customer().register(Customer);

    s3.product().find_by_id(1);
    s3.product().list_all();
    s3.product().create(Product);

    s3.order().find_by_id(1);
    s3.order().list_by_customer(1);
    s3.order().create(Order);
}
