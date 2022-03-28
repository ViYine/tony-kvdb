mod memory;
pub use memory::Memtable;
// Storage trait
// storage trait 能够抽象出存储系统的接口，以便在不同的存储系统中，可以实现不同的存储系统
use crate::{KvError, Kvpair, Value};
// use anyhow::Result;
/// 对存储系统的抽象，不用关心具体数据存在哪里，但 需要定义 外界如何 与存储系统打交道
pub trait Storage {
    /// 从 一个 HashTable 中获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从 一个 HashTable 中 设置 一个 key 的 value， 并返回 旧的 value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看 一个 HashTable 中是否存在 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 删除 一个 HashTable 中的 key, 并返回 旧的 value
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    /// 遍历 一个 HashTable ， 返回所有的 kv pair（接口不友好？）
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;

    /// 遍历 一个 HashTable ， 返回 kv pair 的 迭代器(使用 trait object 统一表示不同的 iterator，只要关联类型为 Kvpair 即可)
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

// 在处理客户端请求时，与之打交道的是 Storage trait，而非某个具体的store，这样可以把存储系统的实现和处理请求的逻辑分离开来。未来根据需要在不同的场景下添加不同的store 时， 只需要 为其实现 Storage trait即可。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = Memtable::new();
        test_basic_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = Memtable::new();
        test_get_all(store);
    }

    // #[test]
    // fn memtable_get_iter_should_work() {
    //     let store = Memtable::new();
    //     test_get_iter(store);
    // }

    fn test_get_iter(store: impl Storage) {
        store.set("test", "key1".into(), "value1".into());
        store.set("test", "key2".into(), "value2".into());

        let mut iter = store.get_iter("test").unwrap();
        assert_eq!(iter.next(), Some(Kvpair::new("key1", "value1".into())));
        assert_eq!(iter.next(), Some(Kvpair::new("key2", "value2".into())));
        assert_eq!(iter.next(), None);
    }

    fn test_basic_interface(store: impl Storage) {
        // 第一次set 时 会创建table， 插入key 并返回 None
        let v = store.set("t1", "hello".into(), "world".into());
        assert!(v.unwrap().is_none());

        // 第二次set 时 会更新key 并返回旧的value
        let v1 = store.set("t1", "hello".into(), "world2".into());
        assert_eq!(v1, Ok(Some("world".into())));

        // get 时会返回 key 对应的 最新的 value
        let v2 = store.get("t1", "hello");

        assert_eq!(v2, Ok(Some("world2".into())));

        // get 不存在的key 会返回 None
        let v3 = store.get("t1", "world");
        assert_eq!(v3, Ok(None));

        // contains 时会返回 key 对应的 value 是否存在
        let v4 = store.contains("t1", "hello");
        assert_eq!(v4, Ok(true));

        // contains 不存在的key 会返回 false
        let v5 = store.contains("t1", "world");
        assert_eq!(v5, Ok(false));

        // del 时会删除 key 对应的 value，并返回旧的 value
        let v6 = store.del("t1", "hello");
        assert_eq!(v6, Ok(Some("world2".into())));

        // del 不存在的key 会返回 None
        let v7 = store.del("t1", "world");
        assert_eq!(v7, Ok(None));

        // del 不存在 的 table 会返回 None
        let v8 = store.del("t2", "world");
        assert_eq!(v8, Ok(None));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t1", "hello".into(), "world".into()).unwrap();
        store.set("t1", "hello2".into(), "world2".into()).unwrap();
        store.set("t1", "hello3".into(), "world3".into()).unwrap();

        let mut v = store.get_all("t1").unwrap();
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            v,
            vec![
                Kvpair {
                    key: "hello".into(),
                    value: Some("world".into())
                },
                Kvpair {
                    key: "hello2".into(),
                    value: Some("world2".into())
                },
                Kvpair {
                    key: "hello3".into(),
                    value: Some("world3".into())
                }
            ]
        );
    }
}
