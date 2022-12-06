use std::collections::HashMap;
use veruna_kernel::sites::{Site, SiteId, SiteIdBuilderImpl, SiteReadOption, SiteRepository as SiteRepositoryContract};
use veruna_kernel::sites::site_kit::SiteKit;
use shaku::{Component, Provider};
use std::ops::Deref;

#[derive(Component)]
#[shaku(interface = veruna_kernel::sites::SiteRepository)]
pub struct SiteRepositoryImpl {
    sites: HashMap<u8, Box<dyn Site>>,
}

impl SiteRepositoryContract for SiteRepositoryImpl {
    fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId> {
        self.sites.insert(42, site);
        let builder = SiteKit::site_id_builder();
        builder.build(42)
    }

    fn read(&self, read_by: SiteReadOption) -> (&Box<dyn Site>, Box<dyn SiteId>) {
        match read_by {
            SiteReadOption::SiteId(id) => {
                let id = id.value();
                let site = self.sites.get(&id).unwrap();
                let builder = SiteKit::site_id_builder();
                let site_id = builder.build(id);
                let result = (site, site_id);

                result
            }
            SiteReadOption::Domain(_) => {
                let id = 42;
                let site = self.sites.get(&id).unwrap();
                let builder = SiteKit::site_id_builder();
                let site_id = builder.build(id);
                let result = (site, site_id);
                result
            }
        }
    }

    fn delete(&self, site_id: Box<dyn Site>) -> bool {
        todo!()
    }
}