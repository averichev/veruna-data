use veruna_kernel::sites::{Site, SiteId, SiteReadOption, SiteRepository as SiteRepositoryContract};
use shaku::{Provider};

#[derive(Provider)]
#[shaku(interface = SiteRepositoryContract)]
pub struct SiteRepository {}

impl SiteRepositoryContract for SiteRepository {
    fn create(&self, site: Box<dyn Site>) -> Box<dyn SiteId> {
        todo!()
    }

    fn read(&self, read_by: SiteReadOption) -> (Box<dyn Site>, Box<dyn SiteId>) {
        todo!()
    }

    fn delete(&self, site_id: Box<dyn Site>) -> bool {
        todo!()
    }
}