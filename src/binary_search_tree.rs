type Link<T> = Option<Box<BinaryTree<T>>>;

pub struct BinaryTree<T> {
    root: Option<T>,
    left: Link<T>,
    right: Link<T>,
}

impl<T> BinaryTree<T>
where
    T: Eq + PartialOrd,
{
    pub fn new(root: Option<T>) -> Self {
        match root {
            Some(root) => Self {
                root: Some(root),
                left: None,
                right: None,
            },
            None => Self {
                root: None,
                left: None,
                right: None,
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // 20 -> 12 -> 33 -> 14 -> 16 -> 22 -> 35 -> 10 -> 12
    pub fn insert(&mut self, item: T) {
        // check root
        let Some(ref root) = self.root else {
            self.root = Some(item);
            return;
        };

        if &item < root {
            if let Some(mut left) = self.left.take() {
                left.insert(item);
                self.left = Some(left); // re-attach self.left

            // leaf
            } else {
                self.left = Some(Box::new(BinaryTree::new(Some(item))));
            }
        } else {
            if let Some(mut right) = self.right.take() {
                right.insert(item);
                self.right = Some(right);
            // leaf
            } else {
                self.right = Some(Box::new(BinaryTree::new(Some(item))));
            }
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        // base case 1
        // new let-else block instead of if let
        let Some(ref root) = self.root else {
            return false;
        };

        // base case 2
        // now I can still use root if it exists after the let-else
        if item == root {
            return true;
        }

        // else: recurse
        let mut left = false;
        let mut right = false;
        if item < root {
            if let Some(left_subtree) = &self.left {
                left = left_subtree.contains(item);
            }
        } else {
            if let Some(ref right_subtree) = self.right {
                right = right_subtree.contains(item);
            }
        }

        left || right
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_bst() {
        let mut bst = BinaryTree::new(Some(1));
        assert_eq!(bst.root.unwrap(), 1);
        bst.insert(2);
        bst.insert(3);
    }

    #[test]
    fn test_insert() {
        let mut bst = BinaryTree::new(Some(20));
        bst.insert(12);
        bst.insert(33);
        bst.insert(14);
        bst.insert(16);
        bst.insert(22);
        bst.insert(35);
        bst.insert(10);
        bst.insert(12);

        assert_eq!(bst.root.unwrap(), 20);
        assert_eq!(bst.left.as_ref().unwrap().root.unwrap(), 12);
        assert_eq!(
            bst.left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .root
                .unwrap(),
            10
        );
        assert_eq!(bst.root.unwrap(), 20);
    }

    #[test]
    fn test_contains() {
        let mut bst = BinaryTree::new(Some(20));
        bst.insert(12);
        bst.insert(33);
        bst.insert(14);
        bst.insert(16);
        bst.insert(22);
        bst.insert(35);
        bst.insert(10);
        bst.insert(12);

        assert_eq!(bst.contains(&10), true);
        assert_eq!(bst.contains(&1), false);
        assert_eq!(bst.contains(&8), false);
        assert_eq!(bst.contains(&0), false);
        assert_eq!(bst.contains(&20), true);
        assert_eq!(bst.contains(&33), true);
        assert_eq!(bst.contains(&12), true);
        assert_eq!(bst.contains(&12), true);
        assert_eq!(bst.contains(&22), true);
        assert_eq!(bst.contains(&16), true);
        assert_eq!(bst.contains(&35), true);
    }
}
