use rand::RngExt;

pub enum SortAlgorithm {
    BubbleSort,
}

#[derive(Debug, Default)]
pub struct Data {
    pub id: usize,
    pub value: u8,
}

pub struct SortableArray {
    pub data: Vec<Data>,
    sorted: bool,
    pub sort_algorithm: SortAlgorithm,
    pub min: u8,
    pub max: u8,
}

impl SortableArray {
    pub fn new(data: Vec<Data>, sort_algorithm: SortAlgorithm) -> Self {
        let min = data.iter().map(|d| d.value).min().unwrap_or(0);
        let max = data.iter().map(|d| d.value).max().unwrap_or(0);

        Self {
            data,
            sorted: false,
            sort_algorithm,
            min,
            max,
        }
    }

    pub fn generate_random_data(
        size: usize,
        sort_algorithm: SortAlgorithm,
        max: u8,
        min: u8,
    ) -> Self {
        let mut data = Vec::new();
        let mut rng = rand::rng();

        for i in 0..size {
            let value: u8 = rng.random_range::<u8, _>(min..max);
            data.push(Data { id: i, value });
        }

        Self::new(data, sort_algorithm)
    }

    pub fn length(&self) -> usize {
        self.data.len()
    }

    pub fn shuffle_new_set(&mut self, size: Option<usize>) {
        let mut data = Vec::new();
        let mut rng = rand::rng();
        let size = size.unwrap_or(self.data.len());

        for i in 0..size {
            let value: u8 = rng.random_range::<u8, _>(self.min..self.max);
            data.push(Data { id: i, value });
        }

        self.data = data;
        self.sorted = false;
    }

    pub fn sort(&mut self) {
        match self.sort_algorithm {
            SortAlgorithm::BubbleSort => self.bubble_sort(),
        }
    }

    pub fn is_sorted(&self) -> bool {
        self.sorted
    }

    fn bubble_sort(&mut self) {
        for i in 0..self.data.len() {
            for j in 0..self.data.len() - i - 1 {
                if self.data[j].value > self.data[j + 1].value {
                    self.data.swap(j, j + 1);
                }
            }
        }
        self.sorted = true;
    }
}
