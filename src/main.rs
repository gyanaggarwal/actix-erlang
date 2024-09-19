use actix::prelude::*;
use std::collections::HashMap;
use rand::prelude::*;

use actix_erlang::actix_erl::{CrdtActor, Receipients, UserMsg};

#[actix::main]
async fn main() {
    test_crdt().await;
    System::current().stop();
}

async fn test_crdt() {
    let mut rmap: HashMap<u16, Addr<CrdtActor>>= HashMap::new();

    let crdt0 = CrdtActor::new(0);
    let crdt1 = CrdtActor::new(1);
    let crdt2 = CrdtActor::new(2);

    let addr0 = crdt0.start();
    let addr1 = crdt1.start();
    let addr2 = crdt2.start();
    rmap.insert(0, addr0.clone());
    rmap.insert(1, addr1.clone());
    rmap.insert(2, addr2.clone());

    let rcpts = Receipients::new(rmap);

    let _ = addr0.send(rcpts.clone()).await;
    let _ = addr1.send(rcpts.clone()).await;
    let _ = addr2.send(rcpts.clone()).await;

    for _i in 0..20 {
        let node_index = get_rand(0, 3);
        let value = get_rand(1, 20) as u64;
        let user_msg = UserMsg::new(value);

        match node_index {
            0 => addr0.send(user_msg).await.unwrap(),
            1 => addr1.send(user_msg).await.unwrap(),
            2 => addr2.send(user_msg).await.unwrap(),
            _ => panic!()
        }
    }
}

fn get_rand(low: u16, high: u16) -> u16 {
    rand::thread_rng().gen_range(low..high)
}
