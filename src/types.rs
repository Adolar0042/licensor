pub struct LicenseReplace {
    pub year: Option<&'static str>,
    pub name: Option<&'static str>,
}

#[allow(dead_code)]
pub struct License {
    pub id: &'static str,
    pub replace: Option<LicenseReplace>,
    pub copyright: Option<&'static str>,
    pub optional: Option<&'static [&'static str]>,
}

#[allow(dead_code)]
pub struct Exception {
    pub id: &'static str,
    pub with: Option<&'static [&'static str]>,
    pub optional: Option<&'static [&'static str]>,
}
