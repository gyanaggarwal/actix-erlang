use actix::prelude::*;
use crate::{CounterCrdtValue, CounterOpsValue, NodeType};
use crate::crdt::{CRDT, CrdtTrait, OpsInstance, UserMsg, PeerMsg, SDPOpsType};

#[derive(Debug, Clone, PartialEq)]
pub struct GCounter;

#[derive(Debug, Clone, PartialEq)]
pub struct CounterCrdtStruct(pub CounterCrdtValue);
impl CounterCrdtStruct {
    pub fn new() -> Self {
        Self(0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CounterOpsStruct(pub CounterOpsValue);
impl CounterOpsStruct {
    pub fn new(ops: CounterOpsValue) -> Self {
        Self(ops)
    }
}

impl CrdtTrait<CounterCrdtStruct, CounterOpsStruct> for CRDT<CounterCrdtStruct, CounterOpsStruct, GCounter> {
    fn process_ops_instance(&mut self, ops_instance: &OpsInstance<CounterOpsStruct>) {
        self.crdt_value.0 += ops_instance.ops_value.0;
    }
}

impl Handler<UserMsg<CounterOpsStruct>> for CRDT<CounterCrdtStruct, CounterOpsStruct, GCounter> {
    type Result = ();

    fn handle(&mut self, msg: UserMsg<CounterOpsStruct>, _ctx: &mut Self::Context) -> Self::Result {
        self.process_ops_instance(&msg.ops_instance);
        if let Some(rec) = self.receipients.clone() {
            let peer_msg = PeerMsg::new(msg.clone());
            for (_, addr) in rec {
                addr.do_send(peer_msg.clone());
            }
        }
        println!("self.handler.user_msg {:?} {:?} {:?}", self.state, self.node, self.query());
    }
}

impl Handler<PeerMsg<CounterOpsStruct>> for CRDT<CounterCrdtStruct, CounterOpsStruct, GCounter> {
    type Result = ();

    fn handle(&mut self, msg: PeerMsg<CounterOpsStruct>, _ctx: &mut Self::Context) -> Self::Result {
        self.process_ops_instance(&msg.user_msg.ops_instance);  
        println!("self.handler.peer_msg {:?} {:?} {:?}", self.state, self.node, self.query());
    }
}

pub fn user_msg(node: NodeType, ops_type: SDPOpsType, ops_value: CounterOpsValue) -> UserMsg<CounterOpsStruct> {
    UserMsg::new(node, ops_type, CounterOpsStruct::new(ops_value))
}

