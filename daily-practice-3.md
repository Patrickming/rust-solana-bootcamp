# 习题3

### 题目
有一个简单的链表，
其实现代码见：[https://github.com/lbc-team/rust-solana-bootcamp/blob/main/linkedlist.rs](https://github.com/lbc-team/rust-solana-bootcamp/blob/main/linkedlist.rs)，
请完成以下几个方法的具体实现，使其能通过测试用例。
1. `fn is_empty(&self) -> bool` 方法
2. `fn peek_mut(&mut self) -> Option<&mut T>` 方法
3. `impl<T> Iterator for IntoIter<T> `中的 `next` 方法
4. `impl<'a,T> Iterator for Iter<'a, T>` 中的 `next` 方法
5. `impl<'a,T> Iterator for IterMut<'a,T> ` 中的 `next` 方法

### 目的
通过本题目中的源代码阅读与的练习，可以加深`迭代器`、`生命周期的标注`、`Option<T>（熟悉其as_ref、as_deref、as_deref_mut、take等方法的使用）`、`Box<T>`以及`&T、&mut T`等理解。
所以，在阅读源代码的时候，可以对上面提到的这些知识，可以关联起来学习。
