#[cfg(windows)]
pub(crate) fn symlink_file(original: &str, link: &str) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(original, link)
}
