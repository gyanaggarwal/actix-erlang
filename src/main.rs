use actix::prelude::*;
use std::collections::HashMap;

use actix_erlang::actix_erl::{CrdtActor, Receipients};

#[actix::main]
async fn main() {
    let _ = test_crdt().await;
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

    addr0.send(rcpts.clone()).await.unwrap();
    addr1.send(rcpts.clone()).await.unwrap();
    addr2.send(rcpts.clone()).await.unwrap();
}