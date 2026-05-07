use std::{collections::HashMap, ptr};

#[derive(Default)]
pub struct World {
    data: HashMap<char, isize>
}
impl World {
    pub fn add_data(&mut self, name: char, value: isize) {
        self.data.insert(name, value);
    }

    pub fn get_data(&self, name: &char) -> &isize {
        let r = self.data.get(name).unwrap();
        let ptr = ptr::from_ref(r);

        unsafe { &*ptr }
    }

    pub fn get_data_mut(&mut self, name: &char) -> &mut isize {
        self.data.get_mut(name).unwrap()
    }
}
