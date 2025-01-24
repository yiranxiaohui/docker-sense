use std::path::Path;

fn main() {
    match tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .out_dir(Path::new("src/features/grpc/gen"))
        .compile_protos(
        &["docker.proto"], &["../proto"]) {
        Ok(_ok) => {
            println!("生成proto成功！");
        }
        Err(err) => {
            println!("err = {:?}", err);
        }
    }
}