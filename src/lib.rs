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
}

// /// Trait with some fucntions common in functional programming.
// /// Also contains mutable versions.
// pub trait Func<T>{
//     /// Apply a function on each element in the collection.
//     /// When called on collection, will mutate each element in place.
//     fn map_mut<F>(&mut self, f: &F)
//         where F: Fn(T) -> T;
//     /// Apply a function on each element in the collection.
//     /// Consumes the collection, mutates each element in place, then returns the collection.
//     fn map<F>(self, f: &F) -> Self
//         where F: Fn(T) -> T;
//     /// Only keeps elements that the function returns true for.
//     /// Will filter the collection in place, should retain order.
//     fn filter_mut<F>(&mut self, f: &F)
//         where F: Fn(&T) -> bool;
//     /// Only keeps elements that the function returns true for.
//     /// Consumes the collection, mutates it in place, then returns it.
//     fn filter<F>(self, f: &F) -> Self
//         where F: Fn(&T) -> bool;
//     /// Only keeps elements that the function returns true for.
//     /// Will filter the collection in place.
//     /// Works in O(n), but does not retain the order.
//     fn filter_swap_mut<F>(&mut self, f: &F)
//         where F: Fn(&T) -> bool;
//     fn filter_swap<F>(self, f: &F) -> Self
//         where F: Fn(&T) -> bool;
//     /// Folds a collections elements with a function, returning a single value.
//     fn fold<F,R>(&self, f: &F, v: R) -> R
//         where F: Fn(R,T) -> R;
//     /// Returns true when all elements return true for the predicate.
//     fn all<F>(&self, f: &F) -> bool
//         where F: Fn(&T) -> bool;
// }
//
// /// Implementation of ```Func``` for ```Vec<T: Copy>```.
// impl<T: Copy> Func<T> for Vec<T>{
//     /// # Example
//     /// ```
//     /// use fnrs::Func;
//     /// let mut v = vec![0,1,2];
//     /// v.map_mut(&|x| x + 1);
//     /// assert_eq!(v,vec![1,2,3]);
//     /// ```
//     fn map_mut<F>(&mut self, f: &F)
//     where F: Fn(T) -> T{
//         let len = self.len();
//         for i in 0..len{
//             self[i] = f(self[i]);
//         }
//     }
//     /// # Example
//     /// ```
//     /// use fnrs::Func;
//     /// assert_eq!(vec![0,1,2].map(&|x| x + 1), vec![1,2,3]);
//     /// ```
//     fn map<F>(mut self, f: &F) -> Self
//     where F: Fn(T) -> T{
//         self.map_mut(f);
//         self
//     }
//     /// # Example
//     /// ```
//     /// use fnrs::Func;
//     /// assert_eq!(vec![1,2,3,4].fold(&|r,x| r * x, 1), 24);
//     /// ```
//     fn fold<F,R>(&self, f: &F, mut v: R) -> R
//     where F: Fn(R,T) -> R{
//         for x in self{
//             v = f(v,*x);
//         }
//         v
//     }
//     /// # Example
//     /// ```
//     /// use fnrs::Func;
//     /// assert_eq!(vec![3,4,5].all(&|x| x > &2), true);
//     /// assert_eq!(vec![5,3,4].all(&|x| x > &3), false);
//     /// ```
//     fn all<F>(&self, f: &F) -> bool
//     where F: Fn(&T) -> bool{
//         for x in self{
//             if !f(x) { return false; }
//         }
//         true
//     }
//     /// # Example
//     /// ```
//     /// use fnrs::Func;
//     /// let mut v = vec![3,4,2,1,9,2,3];
//     /// v.filter_mut(&|x| x >= &3);
//     /// assert_eq!(v, vec![3,4,9,3]);
//     /// ```
//     fn filter_mut<F>(&mut self, f: &F)
//     where F: Fn(&T) -> bool{
//         if self.is_empty() { return; }
//         let mut i = 0;
//         while i < self.len(){
//             if f(&self[i]) {
//                 i += 1;
//                 continue;
//             }
//             self.remove(i);
//         }
//     }
//     /// # Example
//     /// ```
//     /// use fnrs::Func;
//     /// assert_eq!(vec![0,1,2,3,4,5].filter(&|x| x % 2 == 0), vec![0,2,4]);
//     /// ```
//     fn filter<F>(mut self, f: &F) -> Self
//     where F: Fn(&T) -> bool{
//         self.filter_mut(f);
//         self
//     }
//     /// # Example
//     /// ```
//     /// use fnrs::Func;
//     /// let mut v = vec![true,false,false,true,false];
//     /// v.filter_swap_mut(&|x| *x);
//     /// assert_eq!(v, vec![true,true]);
//     /// ```
//     fn filter_swap_mut<F>(&mut self, f: &F)
//     where F: Fn(&T) -> bool{
//         if self.is_empty() { return; }
//         let mut i = 0;
//         while i < self.len(){
//             if f(&self[i]) {
//                 i += 1;
//                 continue;
//             }
//             self.swap_remove(i);
//         }
//     }
//     /// # Example
//     /// ```
//     /// use fnrs::Func;
//     /// assert_eq!(vec![true,false,false,true,true,false].filter_swap(&|x| !*x), vec![false; 3])
//     /// ```
//     fn filter_swap<F>(mut self, f: &F) -> Self
//     where F: Fn(&T) -> bool{
//         self.filter_mut(f);
//         self
//     }
//}

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
        if j == slen { true }
        else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    // #[test]
    // fn test_map(){
    //     assert_eq!(map(&vec![2,3,4], &|x| x + 1), vec![3,4,5]);
    // }
    // // Func tests
    // #[test]
    // fn test_func_map_mut(){
    //     let mut v = vec![0,1,2];
    //     v.map_mut(&|x| x + 1);
    //     assert_eq!(v,vec![1,2,3]);
    // }
    // #[test]
    // fn test_func_map(){
    //     assert_eq!(vec![0,1,2].map(&|x| x + 1), vec![1,2,3]);
    // }
    // #[test]
    // fn test_func_fold(){
    //     assert_eq!(vec![1,2,3,4].fold(&|r,x| r * x, 1), 24);
    // }
    // #[test]
    // fn test_func_all(){
    //     assert_eq!(vec![3,4,5].all(&|x| x > &2), true);
    //     assert_eq!(vec![5,3,4].all(&|x| x > &3), false);
    // }
    // #[test]
    // fn test_func_filter_mut(){
    //     let mut v = vec![3,4,2,1,9,2,3];
    //     v.filter_mut(&|x| x >= &3);
    //     assert_eq!(v, vec![3,4,9,3]);
    // }
    // #[test]
    // fn test_func_filter(){
    //     assert_eq!(vec![0,1,2,3,4,5].filter(&|x| x % 2 == 0), vec![0,2,4]);
    // }
    // #[test]
    // fn test_func_filter_swap_mut(){
    //     let mut v = vec![true,false,false,true,false];
    //     v.filter_swap_mut(&|x| *x);
    //     assert_eq!(v, vec![true,true]);
    // }
    // #[test]
    // fn test_func_filter_swap(){
    //     assert_eq!(vec![true,false,false,true,true,false].filter_swap(&|x| !*x), vec![false; 3])
    // }
    // // Sequence tests
    // #[test]
    // fn test_sequence_has_seq(){
    //     assert_eq!(vec![1,2,1,3,5,9,0].has_seq(&vec![1,3,5]), true);
    //     assert_eq!(vec![1,2,1,3,5,9,0].has_seq(&vec![1,3,5,8]), false);
    // }
}
