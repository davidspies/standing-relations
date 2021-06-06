use crate::InputSender;

impl<D> InputSender<'_, (D, isize)> {
    pub fn update(&self, x: D, r: isize) {
        self.send((x, r))
    }
    pub fn add(&self, x: D) {
        self.update(x, 1)
    }
    pub fn remove(&self, x: D) {
        self.update(x, -1)
    }
}
