include!(concat!(env!("OUT_DIR"), "/version.rs"));

pub(crate) fn get() -> &'static str {
    VERSION
}
