use actix::prelude::*;
use std::collections::HashMap;
use rand::prelude::*;

use actix_erlang::crdt::{CRDT, CrdtType, SDPOpsType, Receipients};
use actix_erlang::gcnt_crdt::{self, GCounter, CounterCrdtStruct, CounterOpsStruct};
use actix_erlang::gset_crdt::{self, GSet, SetCrdtStruct, SetOpsStruct};

#[actix::main]
async fn main() {
    let _ = test_gcounter().await;
    let _ = test_gset().await;
    System::current().stop();
}

async fn test_gcounter () {
    let mut rmap = HashMap::new();

    let crdt_value = CounterCrdtStruct::new();

    let node0: u16 = 0;
    let node1: u16 = 1;
    let node2: u16 = 2;

    let crdt0: CRDT<CounterCrdtStruct, CounterOpsStruct, GCounter> = CRDT::new(node0, crdt_value.clone(), CrdtType::GCounterCrdt);
    let crdt1: CRDT<CounterCrdtStruct, CounterOpsStruct, GCounter> = CRDT::new(node1, crdt_value.clone(), CrdtType::GCounterCrdt);
    let crdt2: CRDT<CounterCrdtStruct, CounterOpsStruct, GCounter> = CRDT::new(node2, crdt_value.clone(), CrdtType::GCounterCrdt);
 
    let addr0 = crdt0.start();
    let addr1 = crdt1.start();
    let addr2 = crdt2.start();

    rmap.insert(node0, addr0.clone());
    rmap.insert(node1, addr1.clone());
    rmap.insert(node2, addr2.clone());

    let rcpts = Receipients::new(rmap);

    let _ = addr0.send(rcpts.clone()).await;
    let _ = addr1.send(rcpts.clone()).await;
    let _ = addr2.send(rcpts.clone()).await;

    for _i in 0..20 {
        let node_index = get_rand(0, 3) as u16;
        let value = get_rand(1, 20) as u64;
        let user_msg = gcnt_crdt::user_msg(node_index, SDPOpsType::SDPAdd, value);

        match node_index {
            0 => addr0.send(user_msg).await.unwrap(),
            1 => addr1.send(user_msg).await.unwrap(),
            2 => addr2.send(user_msg).await.unwrap(),
            _ => panic!()
        }
    }
}

async fn test_gset () {
    let mut rmap = HashMap::new();

    let crdt_value = SetCrdtStruct::new();

    let node0: u16 = 0;
    let node1: u16 = 1;
    let node2: u16 = 2;

    let crdt0: CRDT<SetCrdtStruct, SetOpsStruct, GSet> = CRDT::new(node0, crdt_value.clone(), CrdtType::GSetCrdt);
    let crdt1: CRDT<SetCrdtStruct, SetOpsStruct, GSet> = CRDT::new(node1, crdt_value.clone(), CrdtType::GSetCrdt);
    let crdt2: CRDT<SetCrdtStruct, SetOpsStruct, GSet> = CRDT::new(node2, crdt_value.clone(), CrdtType::GSetCrdt);
 
    let addr0 = crdt0.start();
    let addr1 = crdt1.start();
    let addr2 = crdt2.start();

    rmap.insert(node0, addr0.clone());
    rmap.insert(node1, addr1.clone());
    rmap.insert(node2, addr2.clone());

    let rcpts = Receipients::new(rmap);

    let _ = addr0.send(rcpts.clone()).await;
    let _ = addr1.send(rcpts.clone()).await;
    let _ = addr2.send(rcpts.clone()).await;

    for _i in 0..20 {
        let node_index = get_rand(0, 3) as u16;
        let value = get_rand(1, 10) as u16;
        let user_msg = gset_crdt::user_msg(node_index, SDPOpsType::SDPAdd, value);

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
