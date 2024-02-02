use std::env;

fn main() {
    // Only add manifest to release.
    // Running the generated exe on the command line in WSL is not possible,
    // and the prompt when double clicking is so long (prombably because it's on its own network)
    // Running it outside of WSL works fine. Therefor, use debug when developing and only release in the pipeline.
    // Maybe add profiles in the future to allow non-manifest release builds?
    if env::var("TARGET").expect("target").contains("windows")
        && env::var("PROFILE").expect("profile") == "release".to_owned() {
        embed_resource::compile("build.rc", embed_resource::NONE);
        println!("cargo:rerun-if-changed=build.rc");
    }
}