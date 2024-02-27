pub struct List<T> {
    size : usize , // 当前链表中包括的元素个数
    head : Link<T>, // 链表头，指向第一个元素（如果存在）
}

// 节点连接用 Box 指针(大小确定)，因为确定大小才能分配内存
type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem : T,
    next : Link<T>,
}



impl<T> List <T> {
    pub fn new() -> Self {
        List { size: 0, head: None }
    }
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn size(&self) -> usize {
        self.size
    }


    // 节点总是加到头部
    pub fn push(&mut self , elem: T) {
        let node = Box::new(Node {
            elem : elem,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    // peek 不改变值，只能是个引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    // peek_mut 可改变值 ， 是可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // 请实现此处的功能
        todo!()
    }


    // 以下是实现的三种迭代功能
    // into_iter : 转换链表为迭代器
    // iter : 链表不变，只得到不可变迭代器
    // iter_mut : 链表不变 ， 得到可变迭代器
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<T> {
        Iter {next: self.head.as_deref()}
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {next: self.head.as_deref_mut() }
    }
}

// 实现三种迭代具体实现
pub struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct Iter<'a, T: 'a> {next: Option<&'a Node<T>>}
impl<'a,T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct IterMut<'a, T: 'a> { next: Option<&'a mut Node<T>> }
impl<'a,T> Iterator for IterMut<'a,T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

// 为链表实现 Drop
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take(); }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basics() {
        let mut list = List ::new();
        list .push(1); list .push(2); list .push(3);
        assert_eq !( list .pop() , Some(3));
        assert_eq!( list .peek(), Some(&2));
        assert_eq !( list .peek_mut() , Some(&mut 2));
        list.peek_mut().map(|val| {
            *val = 4;
        });
        assert_eq!(list.peek(), Some(&4));
        println!("basics test Ok!");
    }

    #[test]
    fn test_into_iter () {
        let mut list = List ::new();
        list .push(1); list .push(2); list .push(3);
        let mut iter = list . into_iter ();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }


    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(1);list.push(2);list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(1);list.push(2);list.push(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);

    }


}