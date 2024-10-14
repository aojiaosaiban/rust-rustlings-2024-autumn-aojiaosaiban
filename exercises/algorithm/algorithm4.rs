use std::cmp::Ordering;  
use std::fmt::Debug;  

#[derive(Debug)]  
struct TreeNode<T>  
where  
    T: Ord,  
{  
    value: T,  
    left: Option<Box<TreeNode<T>>>,  
    right: Option<Box<TreeNode<T>>>,  
}  

#[derive(Debug)]  
struct BinarySearchTree<T>  
where  
    T: Ord,  
{  
    root: Option<Box<TreeNode<T>>>,  
}  

impl<T> TreeNode<T>  
where  
    T: Ord,  
{  
    fn new(value: T) -> Self {  
        TreeNode {  
            value,  
            left: None,  
            right: None,  
        }  
    }  

    // Insert a node into the tree  
    fn insert(&mut self, value: T) {  
        match value.cmp(&self.value) {  
            Ordering::Less => {  
                if let Some(ref mut left_child) = self.left {  
                    left_child.insert(value);  
                } else {  
                    self.left = Some(Box::new(TreeNode::new(value)));  
                }  
            }  
            Ordering::Greater => {  
                if let Some(ref mut right_child) = self.right {  
                    right_child.insert(value);  
                } else {  
                    self.right = Some(Box::new(TreeNode::new(value)));  
                }  
            }  
            Ordering::Equal => {  
                // Do nothing (no duplicates allowed)  
            }  
        }  
    }  
}  

impl<T> BinarySearchTree<T>  
where  
    T: Ord,  
{  
    fn new() -> Self {  
        BinarySearchTree { root: None }  
    }  

    // Insert a value into the BST  
    fn insert(&mut self, value: T) {  
        match self.root {  
            Some(ref mut node) => node.insert(value),  
            None => {  
                self.root = Some(Box::new(TreeNode::new(value)));  
            }  
        }  
    }  

    // Search for a value in the BST  
    fn search(&self, value: T) -> bool {  
        self.root.as_ref().map_or(false, |node| node.search(value))  
    }  
}  

// Implement search for TreeNode  
impl<T> TreeNode<T>  
where  
    T: Ord,  
{  
    fn search(&self, value: T) -> bool {  
        match value.cmp(&self.value) {  
            Ordering::Less => {  
                self.left.as_ref().map_or(false, |node| node.search(value))  
            }  
            Ordering::Greater => {  
                self.right.as_ref().map_or(false, |node| node.search(value))  
            }  
            Ordering::Equal => true,  
        }  
    }  
}  

#[cfg(test)]  
mod tests {  
    use super::*;  

    #[test]  
    fn test_insert_and_search() {  
        let mut bst = BinarySearchTree::new();  

        assert_eq!(bst.search(1), false);  

        bst.insert(5);  
        bst.insert(3);  
        bst.insert(7);  
        bst.insert(2);  
        bst.insert(4);  

        assert_eq!(bst.search(5), true);  
        assert_eq!(bst.search(3), true);  
        assert_eq!(bst.search(7), true);  
        assert_eq!(bst.search(2), true);  
        assert_eq!(bst.search(4), true);  

        assert_eq!(bst.search(1), false);  
        assert_eq!(bst.search(6), false);  
    }  

    #[test]  
    fn test_insert_duplicate() {  
        let mut bst = BinarySearchTree::new();  

        bst.insert(1);  
        bst.insert(1); // Trying to insert a duplicate  

        assert_eq!(bst.search(1), true);  

        match bst.root {  
            Some(ref node) => {  
                assert!(node.left.is_none());  
                assert!(node.right.is_none());  
            },  
            None => panic!("Root should not be None after insertion"),  
        }  
    }  
}


