
use std::str::FromStr;
use regex::Regex;

// 定义一个 包含 parse 方法的 Parse trait
// parse 方法的 第一个 参数与self 无关，是 trait 的静态方法，调用时用：Parse::parse()
// 返回的Self 是 指实现 trait 的 具体类型的类型
trait Parse {
    fn parse(s: &str) -> Option<Self> where Self: Sized;
}

// 为u8 类型实现 Parse trait
// impl Parse for u8 {
//     fn parse(s: &str) -> Option<Self> {
//         let re = Regex::new(r"^[0-9]+").unwrap();
//         match re.captures(s) {
//             Some(captures) => {
//                 captures.get(0)
//                     .map_or(None, |s| s.as_str().parse().unwrap())}
//             None => None
//         }
//     }
// }

// 考虑 实现一个 泛型的 parse 支持 u8, i32, f32 等类型的 字符串解析
// 第1步, 先找到对应的字符串 -> 字符串要能 解析成数字 -> 对应 要实现 str::parse
// 实现 FromStr trait. 就能调用 str::parse，所以第一个泛型参数的约束是 要实现 FromStr trait
// 第2步, 如果 不能正常返回 解析 的 字符串，应该返回什么？0，-1，默认值？
// 每个类型的默认值不一样，所以每个类型要有 自己的 default trait，
// 默认值是 传入的解析的 字符串中有 默认值，还是解析 失败 返回的值，怎么区分？
// 所以，解析失败应该返回一个错误 -> 返回应该是 Option<T>， 或者 是一个 Result<T, E>

// option T 

impl<T: FromStr + Sized> Parse for T{
    fn parse(s: &str) -> Option<Self> {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        match re.captures(s) {
            Some(captures) => {
                captures.get(0)
                    .map_or(None, |s| s.as_str().parse().ok())}
            None => None
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {

        assert_eq!(123u8, u8::parse("123.d").unwrap());
        assert_eq!(123.1, f32::parse("123.1d").unwrap());
        assert_eq!(1230, i32::parse("1230d").unwrap());
        assert_eq!(true, u8::parse("sssd").is_none());
        println!("{:?}", "-12.4".parse::<i32>());
    }
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", u8::parse("123acv"));
}
