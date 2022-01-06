fn main() {
    tonic_build::compile_protos("proto/songs.proto",)
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
    tonic_build::compile_protos("proto/song_infos.proto",)
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}