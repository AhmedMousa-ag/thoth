fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("grpc_proto/mathop.proto")?;
    Ok(())
}
