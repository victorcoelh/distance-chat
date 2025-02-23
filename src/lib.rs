pub mod entities;
pub mod networking;
pub mod types;

pub mod chat_rpc {
    tonic::include_proto!("chat_rpc");
}
