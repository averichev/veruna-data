use std::collections::HashMap;
use veruna_kernel::sites::{Site, SiteId, SiteIdBuilder, SiteIdBuilderImpl, SiteReadOption, SiteRepository as SiteRepositoryContract};

pub struct SiteRepository {
    sites: HashMap<u8, Box<dyn Site>>,
}

impl SiteRepository {
    pub fn new() -> Box<dyn SiteRepositoryContract> {
        let result = SiteRepository { sites: Default::default() };
        Box::new(result)
    }
}

impl SiteRepositoryContract for SiteRepository {
    fn create(&mut self, site: Box<dyn Site>) -> Box<dyn SiteId> {
        self.sites.insert(42, site);
        let builder = SiteIdBuilderImpl::new();
        builder.build(42)
    }

    fn read(&self, read_by: SiteReadOption) -> (&Box<dyn Site>, Box<dyn SiteId>) {
        match read_by {
            SiteReadOption::SiteId(id) => {
                let id = id.value();
                let site = self.sites.get(&id).unwrap();
                let builder = SiteIdBuilderImpl::new();
                let site_id = builder.build(id);
                let result = (site, site_id);

                result
            }
            SiteReadOption::Domain(_) => {
                let id = 42;
                let site = self.sites.get(&id).unwrap();
                let builder = SiteIdBuilderImpl::new();
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