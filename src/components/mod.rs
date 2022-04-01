use wasm_bindgen::JsCast;
use web_sys::{Element, NodeList};

pub mod app;
mod loading;
mod site;

pub trait ElementList {
    fn to_list<T: AsRef<Element> + JsCast>(self) -> Vec<T>;
}

impl ElementList for NodeList {
    fn to_list<T>(self) -> Vec<T>
    where
        T: AsRef<Element> + JsCast,
    {
        let mut result = Vec::with_capacity(self.length() as usize);

        for index in 0..self.length() {
            if let Some(item) = self.get(index) {
                if !item.has_type::<T>() {
                    continue;
                }
                let item = item
                    .dyn_into::<T>()
                    .expect("could not cast node to element");
                result.push(item);
            }
        }

        result
    }
}
