use std::rc::Rc;

pub trait Loadable<T> {
    fn load(path: &str) -> Result<T, std::io::Error>;
}

pub struct ResourceBox<T: Loadable<T>> {
    resource: Option<Rc<T>>,
    path: String,
}

impl<T: Loadable<T>> ResourceBox<T> {
    pub fn new(path: &str) -> ResourceBox<T> {
        Self {
            resource: None,
            path: String::from(path),
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
