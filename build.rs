use prost_build::Config;

fn main() {
    let mut config = Config::new();
    config.bytes(&["."]); // bytes 类型 生成为Bytes类型，非缺省的 Vec<u8>
    config.type_attribute(".", "#[derive(PartialOrd)]"); // 为所有类型都添加了PartialOrd 派生宏
    config
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
