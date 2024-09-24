use actix::prelude::*;
use crate::{SetOpsValue, NodeType};
use crate::crdt::{CRDT, CrdtTrait, OpsInstance, UserMsg, PeerMsg, SDPOpsType};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct GSet;

#[derive(Debug, Clone)]
pub struct SetCrdtStruct(pub HashSet<SetOpsValue>);
impl SetCrdtStruct {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetOpsStruct(pub SetOpsValue);
impl SetOpsStruct {
    pub fn new(ops: SetOpsValue) -> Self {
        Self(ops)
    }
}

impl CrdtTrait<SetCrdtStruct, SetOpsStruct> for CRDT<SetCrdtStruct, SetOpsStruct, GSet> {
    fn process_ops_instance(&mut self, ops_instance: &OpsInstance<SetOpsStruct>) {
        self.crdt_value.0.insert(ops_instance.ops_value.0);
    }
}

impl Handler<UserMsg<SetOpsStruct>> for CRDT<SetCrdtStruct, SetOpsStruct, GSet> {
    type Result = ();

    fn handle(&mut self, msg: UserMsg<SetOpsStruct>, _ctx: &mut Self::Context) -> Self::Result {
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

impl Handler<PeerMsg<SetOpsStruct>> for CRDT<SetCrdtStruct, SetOpsStruct, GSet> {
    type Result = ();

    fn handle(&mut self, msg: PeerMsg<SetOpsStruct>, _ctx: &mut Self::Context) -> Self::Result {
        self.process_ops_instance(&msg.user_msg.ops_instance);  
        println!("self.handler.peer_msg {:?} {:?} {:?}", self.state, self.node, self.query());
    }
}

pub fn user_msg(node: NodeType, ops_type: SDPOpsType, ops_value: SetOpsValue) -> UserMsg<SetOpsStruct> {
    UserMsg::new(node, ops_type, SetOpsStruct::new(ops_value))
}

