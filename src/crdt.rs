use actix::prelude::*;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::NodeType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CrdtType {
    GSetCrdt,
    GCounterCrdt
}

pub trait CrdtTrait<CrdtValue, OpsValue: Clone+PartialEq> {
    fn process_ops_instance(&mut self, ops_instance: &OpsInstance<OpsValue>);
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SDPOpsType {
    SDPAdd,
    SDPMult,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OpsInstance<OpsValue: Clone+PartialEq> {
    pub ops_type: SDPOpsType,
    pub ops_value: OpsValue
}
impl <OpsValue: Clone+PartialEq> OpsInstance<OpsValue> {
    pub fn new(ops_type: SDPOpsType, ops_value: OpsValue) -> Self {
        Self {ops_type, ops_value}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Message)]
#[rtype(result = "()")]
pub struct UserMsg<OpsValue: Clone+PartialEq>{
    pub node: NodeType,
    pub ops_instance: OpsInstance<OpsValue>
}
impl <OpsValue: Clone+PartialEq> UserMsg<OpsValue> {
    pub fn new(node: NodeType, ops_type: SDPOpsType, ops_value: OpsValue) -> Self{
        let ops_instance = OpsInstance::new(ops_type, ops_value);
        Self{node, ops_instance}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Message)]
#[rtype(result = "()")]
pub struct PeerMsg<OpsValue: Clone+PartialEq>{
    pub user_msg: UserMsg<OpsValue>
}
impl <OpsValue: Clone+PartialEq> PeerMsg<OpsValue> {
    pub fn new(user_msg: UserMsg<OpsValue>) -> Self {
        Self{user_msg}
    }
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Receipients<CrdtValue: Clone+std::marker::Unpin+'static, 
                       OpsValue: Clone+PartialEq+std::marker::Unpin+'static,
                       State: Clone+PartialEq+std::marker::Unpin+'static>  {
    pub rcpts: HashMap<NodeType, Addr<CRDT<CrdtValue, OpsValue, State>>>
}

impl <CrdtValue: Clone+std::marker::Unpin+'static, 
      OpsValue: Clone+PartialEq+std::marker::Unpin+'static,
      State: Clone+PartialEq+std::marker::Unpin+'static> 
      Receipients<CrdtValue, OpsValue, State> {
    pub fn new(rcpts: HashMap<NodeType, Addr<CRDT<CrdtValue, OpsValue, State>>>) -> Self {
        Self{rcpts}
    }
}

#[derive(Debug, Clone)]
pub struct CRDT <CrdtValue: Clone+std::marker::Unpin+'static, 
                 OpsValue: Clone+PartialEq+std::marker::Unpin+'static,
                 State: Clone+PartialEq+std::marker::Unpin+'static> {
    pub node: NodeType,
    pub crdt_type: CrdtType,
    pub crdt_value: CrdtValue,
    pub receipients: Option<HashMap<NodeType, Addr<CRDT<CrdtValue, OpsValue, State>>>>,
    pub state: std::marker::PhantomData<State>
}

impl <CrdtValue: Clone+std::marker::Unpin+'static, 
      OpsValue: Clone+PartialEq+std::marker::Unpin+'static,
      State: Clone+PartialEq+std::marker::Unpin+'static> 
      CRDT<CrdtValue, OpsValue, State> {
    pub fn new(node: NodeType, crdt_value: CrdtValue, crdt_type: CrdtType) -> Self {
        Self{node, 
             crdt_value, 
             crdt_type, 
             receipients: None,
             state: std::marker::PhantomData::<State>}
    }

    pub fn query(&self) -> CrdtValue {
        self.crdt_value.clone()
    }

    pub fn crdt_type(&self) -> CrdtType {
        self.crdt_type.clone()
    }
} 

impl <CrdtValue: Clone+std::marker::Unpin+'static, 
      OpsValue: Clone+PartialEq+std::marker::Unpin+'static,
      State: Clone+PartialEq+std::marker::Unpin+'static> 
    Actor for CRDT<CrdtValue, OpsValue, State> {
    type Context = Context<Self>;
}

impl <CrdtValue: Clone+std::marker::Unpin+'static, 
      OpsValue: Clone+PartialEq+std::marker::Unpin+'static,
      State: Clone+PartialEq+std::marker::Unpin+'static> 
      Handler<Receipients<CrdtValue, OpsValue, State>> for CRDT<CrdtValue, OpsValue, State> {
    type Result = ();

    fn handle(&mut self, msg: Receipients<CrdtValue, OpsValue, State>, _ctx: &mut Self::Context) -> Self::Result {
        let fmap: HashMap<NodeType, Addr<CRDT<CrdtValue, OpsValue, State>>> = msg.rcpts.iter()
            .filter_map(|(rnode, value)|{
                if *rnode != self.node {
                    Some((*rnode, value.clone()))
                } else {
                    None
                }
            })
            .collect();
        self.receipients = Some(fmap);

        println!("self.handle.rcpts {:?} {:?}", self.node, self.receipients);
    }
}




