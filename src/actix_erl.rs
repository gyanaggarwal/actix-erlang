use actix::prelude::*;

use std::collections::HashMap;

type NodeType = u16;
type RecValue = Addr<CrdtActor>;
type NRMap = HashMap<NodeType, RecValue>;

#[derive(Message, Clone, Debug)]
#[rtype(result = "()")]
pub struct UserMsg {
    pub msg_value: u64
}

impl UserMsg {
    pub fn new(msg_value: u64) -> Self {
        Self{msg_value}
    }
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct PeerMsg {
    pub peer_value: u64
}

impl PeerMsg {
    pub fn new(peer_value: u64) -> Self {
        Self{peer_value}
    }
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Receipients {
    pub rcpts: NRMap
}

impl Receipients {
    pub fn new(rcpts: NRMap) -> Self {
        Self{rcpts}
    }
}
#[derive(Debug, Clone)]
pub struct CrdtActor {
    pub node: NodeType,
    pub crdt_value: u64,
    pub receipients: Option<NRMap>
}

impl Actor for CrdtActor {
    type Context = Context<Self>;
}

impl CrdtActor {
    pub fn new(node: NodeType) -> Self {
        Self {node, crdt_value: 0, receipients: None}
    }
}

impl Handler<PeerMsg> for CrdtActor {
    type Result = ();

    fn handle(&mut self, msg: PeerMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.crdt_value += msg.peer_value;

        println!("self:handle:peer_msg {:?}", self.crdt_value);
    }
}

impl Handler<UserMsg> for CrdtActor {
    type Result = ();

    fn handle(&mut self, msg: UserMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.crdt_value += msg.msg_value;
        let peer_msg = PeerMsg::new(msg.msg_value); 
        if let Some(rec) = &self.receipients {
            for (_, addr) in rec {
                addr.do_send(peer_msg.clone());
            }
        }

        println!("self:handle:user_msg {:?}", self.crdt_value);
    }
}

impl Handler<Receipients> for CrdtActor {
    type Result = ();

    fn handle(&mut self, msg: Receipients, _ctx: &mut Self::Context) -> Self::Result {
        let fmap: NRMap = msg.rcpts.iter()
            .filter_map(|(rnode, value)|{
                if *rnode != self.node {
                    Some((*rnode, value.clone()))
                } else {
                    None
                }
            })
            .collect();
        self.receipients = Some(fmap);

        println!("self:handle:rcpts {:?}", self);
    }
}