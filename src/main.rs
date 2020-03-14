use lazy_static::lazy_static;
use rand::seq::IteratorRandom;
use redis::{aio::Connection, pipe, Script};

struct RedisConnection(Connection);

impl std::fmt::Debug for RedisConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RedisAsyncConnnection")
    }
}
struct ProducerOptions {
    stream_max_length: u64,
    approx_max_length: bool
}
impl Default for ProducerOptions {
    fn default () -> Self {
        Self {
          stream_max_length:1000,
          approx_max_length:true
        }
    }
}

struct Producer<'a> {
    options: &'a ProducerOptions
}
fn main() {
    println!("Hello, world!");
}
