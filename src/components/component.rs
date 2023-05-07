pub trait Component {
    fn new(name: &String, path: &String, source: &String) -> Self;
    fn sync(&self);
    fn info(&self) -> &ComponentInfo;
}

#[derive(Debug)]
pub struct ComponentInfo {
    pub name: String,
    pub path: String,

    // source_stamp => <source_type>:<source>@<revision>
    // git => git:https://github.com/xxx/yyy.git@3e2a94afebf12df44f33305c6d21fd7cded61771
    // package on lfs => pkg-lfs:https://github.com/xxx/yyy.git@v0.0.1
    // package on http server => pkg-http:https://example-server.com@v0.0.1
    // action => action:@
    // solution => solution:https://github.com/aaa/bbb.git@e60b891c3ce469133a658bd3f521b661426f44e5
    pub source_stamp: String,
    pub source_type: String,
    pub source: String,
    pub revision: String,
}

impl ComponentInfo {
    pub fn new(name: &String, path: &String, source_stamp: &String) -> Self {

        let parts: Vec<&str> = source_stamp.splitn(2, ":").collect();
        let source_type = parts[0];
        let source = parts[1].split('@').next().unwrap();
        let revision: String = parts[1].splitn(2, '@').last().unwrap().into();

        ComponentInfo {
            name: name.to_string(),
            path: path.to_string(),
            source_stamp: source_stamp.to_string(),
            source_type: source_type.to_string(),
            source: source.to_string(),
            revision: revision.to_string(),
        }
    }
}
