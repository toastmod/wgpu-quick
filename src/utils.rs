pub enum Initable<'a, T> {
    Null(&'a mut Option<T>),
    Active(&'a mut T)
}

pub struct InnerInit<T> {
    data: Option<T>
}

impl<T> InnerInit<T> {
    pub fn new(dat: Option<T>) -> Self {
        Self {
            data: dat
        }
    }
    pub fn match_me(&mut self) -> Initable<T> {
        let mb = &mut self.data;
        if match mb {
            None => true,
            Some(d) => {
                return Initable::Active(d)
            }
        } {
            Initable::Null(mb)
        } else {
            unreachable!()
        }
    }
}