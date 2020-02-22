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
    fn map<F>(self, f: &F) -> Self
        where F: Fn(T) -> T;
}

impl<T: Copy> Func<T> for Vec<T>{
    fn map<F>(mut self, f: &F) -> Self
    where F: Fn(T) -> T{
        let len = self.len();
        for i in 0..len{
            self[i] = f(self[i]);
        }
        self
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
    fn test_functional_map(){
        assert_eq!(vec![0,1,2].map(&|x| { x + 1 }), vec![1,2,3]);
    }
}
