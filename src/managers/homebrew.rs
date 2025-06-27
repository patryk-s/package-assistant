use super::PackageManager;

pub(crate) struct Manager;

impl PackageManager for Manager {
    fn name(&self) -> &'static str {
        "brew"
    }
}
