use crate::*;

mod command_service;

/// 对 Command 的处理对象的收益
pub trait CommandService {
    ///   对 Command 的处理 返回结果
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
