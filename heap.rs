trait Heap {
    fn build_max_heap(&mut self);
    fn add_element(&mut self, int);
    fn max_heapify(&mut self, index: uint);
    fn heapsort(&mut self) -> ~[int];
}

impl Heap for ~[int] {
    fn build_max_heap(&mut self) {
        let mut index = (self.len() / 2) + 1;
        while index > 0 {
            index = index - 1;
            self.max_heapify(index);
        }
    }
    fn add_element(&mut self, elem: int) {
        self.push(elem);
    }
    fn max_heapify(&mut self, index: uint) {
        let left = (index * 2) + 1;
        let right = (index * 2) + 2;
        let mut largest = index;
        if (left < self.len() && (self[left] > self[largest])) {
            largest = left;
        }
        if (right < self.len() && (self[right] > self[largest])) {
            largest = right;
        }
        if (largest != index) {
            self.swap(index, largest);
            self.max_heapify(largest);
        }
    }
    fn heapsort(&mut self) -> ~[int] {
        self.build_max_heap();
        let mut result = ~[];
        while self.len() > 0 {
            result.push(self.remove(0));
            self.build_max_heap();
        }
        result
    }
}

fn main() {
    let mut a = ~[5,2,6,3,7];
    println(a.to_str());
    let b = a.heapsort();
    println(b.to_str());
}
