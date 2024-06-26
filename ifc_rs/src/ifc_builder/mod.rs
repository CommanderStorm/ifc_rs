pub(crate) mod building;
pub(crate) mod materials;
pub(crate) mod openings;
pub(crate) mod prelude;
pub(crate) mod project;
pub(crate) mod roofs;
pub(crate) mod site;
pub(crate) mod slabs;
pub(crate) mod spaces;
pub(crate) mod storey;
pub(crate) mod transforms;
pub(crate) mod walls;
pub(crate) mod windows;

use crate::prelude::*;

pub struct ApplicationInfo<'a> {
    pub developer: Person,
    pub version: &'a str,
    pub name: &'a str,
    pub short_name: &'a str,
}

pub struct OwnerInfo<'a> {
    pub owner: Person,
    pub organization_name: &'a str,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::prelude::*;

    pub fn create_builder() -> IfcProjectBuilder {
        IfcProjectBuilder::new(
            ApplicationInfo {
                developer: Person::empty().given_name("Mario"),
                version: "0.0.1",
                name: "IfcBuilderApplication",
                short_name: "builder",
            },
            OwnerInfo {
                owner: Person::empty().given_name("Luigi"),
                organization_name: "Metabuild",
            },
            Person::empty().given_name("Bowser"),
            "IfcBuider Example Project",
        )
    }
}
