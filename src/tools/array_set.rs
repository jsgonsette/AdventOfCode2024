
pub struct ArraySet<const N: usize> {

    min: [isize; N],
    max: [isize; N],
    offsets: [usize; N],
    content: Vec<bool>,
}

impl<const N: usize> ArraySet<N> {

    pub fn new(min: [isize; N], max: [isize; N]) -> ArraySet<N> {

        let mut offsets = [0; N];
        let mut total = 1;
        for i in 0..N {
            offsets[i] = total;
            total *= (max[i] - min[i] +1) as usize;
        }

        ArraySet { min, max, offsets, content: vec![false; total], }
    }

    pub fn test (&self, item: &[isize; N]) -> bool {
        self.content [self.index(item)]
    }

    pub fn set (&mut self, item: &[isize; N]) {
        let index = self.index(item);
        self.content [index] = true;
    }

    pub fn toggle (&mut self, item: &[isize; N]) {
        let index = self.index(item);
        self.content [index] ^= true;
    }

    pub fn count (&self) -> usize {
        self.content.iter().filter(|&x| *x).count()
    }

    fn index (&self, item: &[isize; N]) -> usize {
        (0..N).map(|i| {
            assert!(item [i] >= self.min[i]);
            assert!(item [i] <= self.max[i]);
            (item[i] - self.min[i]) as usize * self.offsets[i]
        }).sum()
    }
}
