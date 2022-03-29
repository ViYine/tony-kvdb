use std::sync::Arc;

use crate::{
    command_request::RequestData, CommandRequest, CommandResponse, KvError, Memtable, Storage,
};

mod command_service;

pub trait CommandService {
    ///   对 Command 的处理 返回结果
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
/// Service  数据结构
pub struct Service<Store = Memtable> {
    inner: Arc<ServiceInner<Store>>,
}

/// Service 实现 clone
impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Service 内部数据结构
pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store: Storage> Service<Store> {
    /// 创建一个 Service
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    /// 获取 Service 的 Store
    pub fn store(&self) -> &Store {
        &self.inner.store
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        dbg!("Got command: {:?}", &cmd);
        // todo 发送 on_received 事件
        let res = dispatch(cmd, &self.inner.store);
        dbg!("Got response: {:?}", &res);
        // todo 发送 on_executed 事件
        res
    }
}

/// 从 Request 中得到 Response，目前处理的是 Hget 和 Hgetall，hset
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hgetall(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        None => KvError::InvalidCommand("No request data".into()).into(),
        _ => KvError::InternalError("Not implemented".into()).into(),
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::{Kvpair, Memtable, Value};

    #[test]
    fn service_should_work() {
        // service 至少包含一个 storage
        let service = Service::new(Memtable::default());

        // service 可以 运行在多线程环境下， clone 是轻量级的
        let cloned = service.clone();

        // 创建一个线程，往 table t1 中插入 key-value pair
        let handle = thread::spawn(move || {
            let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
            let res = cloned.execute(cmd);
            assert_res_ok(res, &[Value::default()], &[]);
        });

        handle.join().unwrap();

        // 在当前线程环境下，读取 table t1 中的 key-value pair
        let cmd = CommandRequest::new_hget("t1", "hello");
        let res = service.execute(cmd);
        assert_res_ok(res, &["world".into()], &[]);
    }

    // 测试成功返回的结果
    fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
        res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "");
        assert_eq!(res.values, values);
        assert_eq!(res.pairs, pairs);
    }

    // 测试失败返回的结果
    fn assert_res_err(res: CommandResponse, status: u32, message: &str) {
        assert_eq!(res.status, status);
        assert!(res.message.contains(message));
        assert_eq!(res.values, &[]);
        assert_eq!(res.pairs, &[]);
    }
}
