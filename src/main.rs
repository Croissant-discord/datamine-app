use croissant_core::{
    error::Result,  
    websocket::*, https::*,
    savings::{ 
        postgres::*, sqlite::{ 
            *, accumulating::*, handle::* 
        }, surrealdb::* 
    }
};


fn start_reading( token: &str ) -> Result<()> {
    
    println!("Hello, World!!");

    Ok(())
}

fn main() {
    println!("{}", start_reading("token").unwrap_err());
}
