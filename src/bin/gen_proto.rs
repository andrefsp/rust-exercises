fn main() {
    tonic_build::configure()
        .out_dir("src/grpcd/")
        .compile(&["src/grpcd/protos/queue.proto"], &["src/grpcd/protos/"])
        .unwrap_or_else(|e| panic!("Failed to compile protos '{:?}'", e));
}
