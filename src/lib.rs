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

pub trait Func<T>{
    fn map_mut<F>(&mut self, f: &F)
        where F: Fn(T) -> T;
    fn map<F>(self, f: &F) -> Self
        where F: Fn(T) -> T;
    fn filter_mut<F>(&mut self, f: &F)
        where F: Fn(&T) -> bool;
    fn filter<F>(self, f: &F) -> Self
        where F: Fn(&T) -> bool;
    fn filter_swap_mut<F>(&mut self, f: &F)
        where F: Fn(&T) -> bool;
    fn fold<F,R>(&self, f: &F, v: R) -> R
        where F: Fn(R,T) -> R;
    fn all<F>(&self, f: &F) -> bool
        where F: Fn(&T) -> bool;
}

impl<T: Copy> Func<T> for Vec<T>{
    fn map_mut<F>(&mut self, f: &F)
    where F: Fn(T) -> T{
        let len = self.len();
        for i in 0..len{
            self[i] = f(self[i]);
        }
    }
    fn map<F>(mut self, f: &F) -> Self
    where F: Fn(T) -> T{
        self.map_mut(f);
        self
    }
    fn fold<F,R>(&self, f: &F, mut v: R) -> R
    where F: Fn(R,T) -> R{
        for x in self{
            v = f(v,*x);
        }
        v
    }
    fn all<F>(&self, f: &F) -> bool
    where F: Fn(&T) -> bool{
        for x in self{
            if !f(x) { return false; }
        }
        true
    }
    fn filter_mut<F>(&mut self, f: &F)
    where F: Fn(&T) -> bool{
        if self.is_empty() { return; }
        let mut i = 0;
        while i < self.len(){
            if f(&self[i]) {
                i += 1;
                continue;
            }
            self.remove(i);
        }
    }
    fn filter<F>(mut self, f: &F) -> Self
    where F: Fn(&T) -> bool{
        self.filter_mut(f);
        self
    }
    fn filter_swap_mut<F>(&mut self, f: &F)
    where F: Fn(&T) -> bool{
        if self.is_empty() { return; }
        let mut i = 0;
        while i < self.len(){
            if f(&self[i]) {
                i += 1;
                continue;
            }
            self.swap_remove(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_map(){
        assert_eq!(map(&vec![2,3,4], &|x| x + 1), vec![3,4,5]);
    }
    #[test]
    fn test_func_map_mut(){
        let mut v = vec![0,1,2];
        v.map_mut(&|x| x + 1);
        assert_eq!(v,vec![1,2,3]);
    }
    #[test]
    fn test_func_map(){
        assert_eq!(vec![0,1,2].map(&|x| x + 1), vec![1,2,3]);
    }
    #[test]
    fn test_func_fold(){
        assert_eq!(vec![1,2,3,4].fold(&|r,x| r * x, 1), 24);
    }
    #[test]
    fn test_func_all(){
        assert_eq!(vec![3,4,5].all(&|x| x > &2), true);
        assert_eq!(vec![5,3,4].all(&|x| x > &3), false);
    }
    #[test]
    fn test_func_filter_mut(){
        let mut v = vec![3,4,2,1,9,2,3];
        v.filter_mut(&|x| x >= &3);
        assert_eq!(v, vec![3,4,9,3]);
    }
    #[test]
    fn test_func_filter(){
        assert_eq!(vec![0,1,2,3,4,5].filter(&|x| x % 2 == 0), vec![0,2,4]);
    }
    #[test]
    fn test_func_filter_swap_mut(){
        let mut v = vec![true,false,false,true,false];
        v.filter_swap_mut(&|x| *x);
        assert_eq!(v, vec![true,true]);
    }
}
