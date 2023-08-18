use amqprs::{
    connection::{Connection, OpenConnectionArguments},
    callbacks::{DefaultConnectionCallback, DefaultChannelCallback}, channel::{QueueDeclareArguments, BasicPublishArguments}, BasicProperties
};
use tokio;
use tokio::io::Error as TError;
use std::{thread, time::{self, UNIX_EPOCH}};


#[tokio::main]
async fn main() -> Result<(), Box<TError>> {
    let conn = Connection::open(&OpenConnectionArguments::new(
        "jackal.rmq.cloudamqp.com",
        5672  ,
        "hasdzydy:hasdzydy",
        "6KUMdtlqdDab3v34owwQmXIpC8Df-daC",
    ))
    .await.unwrap();
    conn.register_callback(DefaultConnectionCallback).await.unwrap();
    
    let ch = conn.open_channel(None).await.unwrap();
    ch.register_callback(DefaultChannelCallback).await.unwrap();
    
    let q_args = QueueDeclareArguments::default()
        .queue(String::from("hello"))
        .durable(false)
        .finish();
    let (queue_name, _, _) = ch.queue_declare(q_args).await.unwrap().unwrap();
    let mut counter : u64 = 0;

    let max_loop: u64 = time::SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() / 10055;
    
    loop {
        let payload = String::from(" {id: ".to_owned() + &counter.to_string() + " message: 'Hello world!'} ").into_bytes();
        let publish_args = BasicPublishArguments::new("", &queue_name);
        let props = BasicProperties::default().with_delivery_mode(2).finish();

        ch.basic_publish( props, payload, publish_args).await.unwrap();
        eprintln!(" [{}] Sent \"Hello World!\"", counter);
        counter += 1;
        thread::sleep(time::Duration::new(1,0));

        println!("{}", max_loop);

        if counter > max_loop {
            break;
        }
    }
    
    conn.close().await.unwrap();
    
    Ok(())
}