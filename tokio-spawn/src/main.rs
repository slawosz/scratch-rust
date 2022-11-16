use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle)
    }

    for handle in handles {
        // Question: where handle (JoinHandler type) is being transformed into result?
        // I suspect that await is some fancy syntactic sugar, but cant find a place yet. 
        // I suppose the result of the block is wrapped into Result when my block is called,
        // but how await performs the block?
        
        // this `Result` may be an `Err` variant, which should be handled
        // handle.await;//.unwrap()
         
        handle.await.unwrap();
    }
}

async fn my_function(i: i32) {
    println!("I am async function {i}");
    let s1 = read_from_database().await;
    println!("{i} First result: {s1}");
    let s2 = read_from_database().await;
    println!("{i} Second result: {s2}");
}

async fn read_from_database() -> String {
    sleep(Duration::from_millis(50)).await;
    "DB Result".to_string() // original code had .to_owned()
}
