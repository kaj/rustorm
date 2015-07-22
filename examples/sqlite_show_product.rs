extern crate rustorm;

use rustorm::query::Query;
use rustorm::query::{Filter,Equality};
use rustorm::pool::ManagedPool;

fn main(){
    let url = "sqlite:///file.db";
    let mut pool = ManagedPool::init(&url, 1);
    let mut db = pool.connect().unwrap();
        
    let dao = Query::select()
        .columns(vec!["name"])
        .from_table(&"product")
        .execute_with_return(db.as_mut());
    
    println!("dao: {:#?}", dao);
}