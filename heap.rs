trait Heap {
    fn build_max_heap(&mut self, offset: uint);
    fn max_heapify(&mut self, index: uint, offset: uint);
    fn heapsort(&mut self);
}

impl Heap for ~[int] {
    fn build_max_heap(&mut self, offset: uint) {
        let length = self.len() - offset;
        let mut index = (length / 2) + 1;
        while index > 0 {
            index = index - 1;
            self.max_heapify(index, offset);
        }
    }
    fn max_heapify(&mut self, index: uint, offset: uint) {
        let length = self.len() - offset;
        let left = (index * 2) + 1;
        let right = (index * 2) + 2;
        let mut largest = index;
        if (left < length && (self[left] > self[largest])) {
            largest = left;
        }
        if (right < length && (self[right] > self[largest])) {
            largest = right;
        }
        if (largest != index) {
            self.swap(index, largest);
        }
    }
    fn heapsort(&mut self) {
        let mut offset = 0;
        let length = self.len();
        self.build_max_heap(offset);
        while offset != length {
            offset = offset + 1;
            let top = self.remove(0);
            self.push(top);
            self.build_max_heap(offset);
        }
    }
}

fn main() {
    let mut a = ~[5,2,532, 12412, 1243215, 3262, 12 ,6,3,7];
    println(a.to_str());
    a.heapsort();
    println(a.to_str());
}
