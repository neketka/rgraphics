use std::rc::Rc;

pub trait Loadable<T> {
    fn load(path: &str) -> Result<T, std::io::Error>;
}

pub struct ResourceBox<'a, T: Loadable<T>> {
    resource: Option<Rc<T>>,
    path: &'a str,
}

impl<'a, T: Loadable<T>> ResourceBox<'a, T> {
    pub fn new(path: &'a str) -> ResourceBox<'a, T> {
        Self {
            resource: None,
            path,
        }
    }

    pub fn load(&mut self) -> Result<Rc<T>, std::io::Error> {
        if let Some(r) = &self.resource {
            return Ok(r.clone());
        }
        let resource = Rc::new(T::load(&self.path)?.into());
        self.resource = Some(resource);
        return Ok(self.resource.clone().unwrap());
    }

    pub fn unload(&mut self) {
        self.resource = None
    }
}
