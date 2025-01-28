extern crate embed_resource;
use winres::WindowsResource;
fn main() {
    let _ = WindowsResource::new()
            .set_resource_file("sirdoon-manifest.rc")
            .compile();
}