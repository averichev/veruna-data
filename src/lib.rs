use veruna_kernel::sites::{Site, SiteId, SiteReadOption, SiteRepository as SiteRepositoryContract};
use veruna_kernel::sites::site_kit::SiteKit;
use shaku::{Component, Provider};

#[derive(Component)]
#[shaku(interface = veruna_kernel::sites::SiteRepository)]
pub struct SiteRepositoryImpl {
    sites: Vec<Box<dyn Site>>,
}

impl SiteRepositoryContract for SiteRepositoryImpl {
    fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId> {
        self.sites.push(site);
        let builder = SiteKit::site_id_builder();
        builder.build(42)
    }

    fn read(&self, read_by: SiteReadOption) -> (Box<dyn Site>, Box<dyn SiteId>) {
        todo!()
    }

    fn delete(&self, site_id: Box<dyn Site>) -> bool {
        todo!()
    }
}