use std::{cell::RefCell, rc::Rc};

use rand::RngExt;

pub trait SortAlgorithm {
    fn new(data: Rc<RefCell<Vec<Data>>>) -> Self;
    fn sort(&mut self);
    fn is_sorted(&self) -> bool;
    fn reset(&mut self, new_data_set: Rc<RefCell<Vec<Data>>>);
}

pub struct BubbleSort {
    data: Rc<RefCell<Vec<Data>>>,
    i: usize,
    j: usize,
}

impl SortAlgorithm for BubbleSort {
    fn new(data: Rc<RefCell<Vec<Data>>>) -> Self {
        Self { data, i: 0, j: 0 }
    }

    fn sort(&mut self) {
        let data_len = self.data.borrow().len();
        for i in 0..data_len {
            for j in 0..data_len - i - 1 {
                let j_value = self.data.borrow()[j].value;
                let j_plus_1_value = self.data.borrow()[j + 1].value;

                if j_value > j_plus_1_value {
                    self.data.borrow_mut().swap(j, j + 1);
                }
            }
        }
    }

    fn is_sorted(&self) -> bool {
        self.data.borrow().iter().copied().is_sorted()
    }

    fn reset(&mut self, new_data_set: Rc<RefCell<Vec<Data>>>) {
        self.i = 0;
        self.j = 0;
        self.data = new_data_set;
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Data {
    pub id: usize,
    pub value: u8,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

pub struct SortableArray<T>
where
    T: SortAlgorithm,
{
    pub data: Rc<RefCell<Vec<Data>>>,
    pub sort_algorithm: T,
    pub length: usize,
    pub min: u8,
    pub max: u8,
}

impl<T> SortableArray<T>
where
    T: SortAlgorithm,
{
    pub fn new(data: Rc<RefCell<Vec<Data>>>) -> Self {
        let min = data.borrow().iter().map(|d| d.value).min().unwrap_or(0);
        let max = data.borrow().iter().map(|d| d.value).max().unwrap_or(0);
        let length = data.borrow().len();

        let sort_algorithm = T::new(data.clone());

        Self {
            data: data.clone(),
            sort_algorithm,
            length,
            min,
            max,
        }
    }

    pub fn generate_random_data(size: usize, max: u8, min: u8) -> Self {
        let mut data = Vec::new();
        let mut rng = rand::rng();

        for i in 0..size {
            let value: u8 = rng.random_range::<u8, _>(min..max);
            data.push(Data { id: i, value });
        }

        Self::new(Rc::new(RefCell::new(data)))
    }

    pub fn shuffle_new_set(&mut self, size: Option<usize>) {
        let mut data = Vec::new();
        let mut rng = rand::rng();
        let size = size.unwrap_or(self.length);

        for i in 0..size {
            let value: u8 = rng.random_range::<u8, _>(self.min..self.max);
            data.push(Data { id: i, value });
        }

        self.data = Rc::new(RefCell::new(data));
        self.sort_algorithm.reset(self.data.clone());
    }

    pub fn sort(&mut self) {
        self.sort_algorithm.sort();
    }

    pub fn is_sorted(&self) -> bool {
        self.sort_algorithm.is_sorted()
    }
}
