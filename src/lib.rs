/// Aplies function over each element in slice and returns a vector of the results in order.
/// # Example
/// ```
/// assert_eq!(fnrs::map(&vec![2,3,4], &|x| x + 1), vec![3,4,5]);
/// ```
pub fn map<T,F>(inp: &[T], f: &F) -> Vec<T>
    where
        F: Fn(&T) -> T,
{
    let mut res = Vec::new();
    for x in inp{
        res.push(f(x));
    }
    res
}

pub trait MutFunc<T>{
    fn map<F>(self, f: F) -> Self
        where F: FnMut(&mut T);
    fn mmap<F>(&mut self, f: F)
        where F: FnMut(&mut T);
    fn conc(self, other: Self) -> Self;
}

impl<T> MutFunc<T> for Vec<T>{
    /// ```
    /// use fnrs::MutFunc;
    /// assert_eq!(vec![3,6,9],vec![1,2,3].map(|x| *x *= 3));
    /// ```
    fn map<F>(mut self, mut f: F) -> Self
    where F: FnMut(&mut T){
        for x in &mut self{
            f(x);
        }
        self
    }
    /// ```
    /// use fnrs::MutFunc;
    /// let mut x = vec![0,1,2];
    /// x.mmap(|x| *x += 1);
    /// assert_eq!(x, vec![1,2,3]);
    /// ```
    fn mmap<F>(&mut self, mut f: F)
    where F: FnMut(&mut T){
        for x in self{
            f(x);
        }
    }
    /// ```
    /// use fnrs::MutFunc;
    /// let x = vec![0,1,2];
    /// let y = vec![0,1,2];
    /// assert_eq!(x.conc(y), vec![0,1,2,0,1,2]);
    /// ```
    fn conc(mut self, other: Self) -> Self{
        self.extend(other);
        self
    }
}

/// Trait for working with sequences.
/// I only kept Vector in mind.
pub trait Sequence{
    /// Does it contain a sub sequence?
    fn has_seq(&self, seq: &Self) -> bool;
}

impl<T: PartialEq> Sequence for Vec<T>{
    /// # Example
    /// ```
    /// use fnrs::Sequence;
    /// assert_eq!(vec![1,2,1,3,5,9,0].has_seq(&vec![1,3,5]), true);
    /// assert_eq!(vec![1,2,1,3,5,9,0].has_seq(&vec![1,3,5,8]), false);
    /// ```
    fn has_seq(&self, seq: &Self) -> bool{
        let slen = seq.len();
        let mut j = 0;
        for a in self{
            if j == slen { return true; }
            let b = &seq[j];
            if a == b{
                j += 1;
            }else{
                j = 0;
            }
        }
        j == slen
    }
}

/// Init lot's of empty vecs easily
/// # Example
/// ```
/// use fnrs::vecs;
/// vecs!(a, b, c, d);
/// a.push(0);
/// b.push(0.0);
/// c.push("aaaa");
/// d.push(vec![0]);
/// ```
#[macro_export]
macro_rules! vecs{
    ($x:ident) => (
        let mut $x = Vec::new();
    );
    ($x:ident, $($y:ident),+) => (
        let mut $x = Vec::new();
        vecs!($($y),+);
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
