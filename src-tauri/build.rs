extern crate embed_resource;

fn main() {
//    embed_resource::compile("fox-manifest.rc", embed_resource::NONE).manifest_optional().unwrap();
    tauri_build::build()
}
