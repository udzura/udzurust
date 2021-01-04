struct OreCounter {
    count: u32,
    max: u32,
}

impl OreCounter {
    fn new(max: u32) -> OreCounter {
        let count = 0;
        OreCounter { count, max }
    }
}

impl Iterator for OreCounter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.count < self.max {
            true => Some(self.count),
            _ => None,
        };
        self.count += 1;
        ret
    }
}

fn main() {
    for i in OreCounter::new(256) {
        println!("cnt: {}", i);
    }

    println!("Sum: {}", OreCounter::new(256).sum::<u32>())
}
