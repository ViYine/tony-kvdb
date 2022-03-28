use crate::*;

impl service::CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(value)) => value.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl service::CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(kvs) => kvs.into(),
            Err(e) => e.into(),
        }
    }
}

impl service::CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(pair) => match store.set(&self.table, pair.key, pair.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{command_request::RequestData, service::CommandService};

    #[test]
    fn hset_should_work() {
        let store = Memtable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        // println!("cloned {:?}", cmd);
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);

        // println!("{:?}", cmd);
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into()], &[])
    }

    #[test]
    fn hget_should_work() {
        let store = Memtable::new();
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());

        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);
    }

    #[test]
    fn hget_with_non_exist_key_should_return_404() {
        let store = Memtable::new();
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_err(res, 404, "Not found");
    }

    #[test]
    fn hgetall_should_work() {
        let store = Memtable::new();
        let cmds = vec![
            CommandRequest::new_hset("score", "u1", 10.into()),
            CommandRequest::new_hset("score", "u2", 20.into()),
        ];

        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);
        let pairs = &[Kvpair::new("u1", 10.into()), Kvpair::new("u2", 20.into())];
        assert_res_ok(res, &[], pairs);
    }

    // 从 Request 中得到 Response， 目前处理 HGET/HSET/HGETALL 的逻辑是一样的，所以可以把它们放在一起
    fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
        match cmd.request_data.unwrap() {
            RequestData::Hget(v) => v.execute(store),
            RequestData::Hset(v) => v.execute(store),
            RequestData::Hgetall(v) => v.execute(store),
            _ => panic!("unexpected request data"),
        }
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
