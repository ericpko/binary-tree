type Link<T> = Option<Box<BinaryTree<T>>>;

pub struct BinaryTree<T> {
    elem: Option<T>,
    left: Link<T>,
    right: Link<T>,
}

impl<T> BinaryTree<T>
where
    T: Eq,
{
    pub fn new(elem: Option<T>, left: Link<T>, right: Link<T>) -> Link<T> {
        Some(Box::new(Self { elem, left, right }))
    }

    pub fn is_empty(&self) -> bool {
        self.elem.is_none()
    }

    pub fn contains(&self, item: &T) -> bool {
        // base case 1
        // new let-else block instead of if let
        let Some(ref elem) = self.elem else {
            return false;
        };

        // base case 2
        // now I can still use elem if it exists after the let-else
        if elem == item {
            return true;
        }

        // else: recurse
        let mut left = false;
        let mut right = false;
        if let Some(left_subtree) = &self.left {
            left = left_subtree.contains(item);
        }
        if let Some(ref right_subtree) = self.right {
            right = right_subtree.contains(item);
        }

        left || right
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_tree() {
        let root = BinaryTree::new(Some(1), None, None);
        assert_eq!(root.unwrap().elem.unwrap(), 1);
    }

    #[test]
    fn test_contains() {
        let root = BinaryTree::new(
            Some(1),
            BinaryTree::new(Some(2), BinaryTree::new(Some(4), None, None), None),
            BinaryTree::new(
                Some(3),
                BinaryTree::new(Some(8), None, None),
                BinaryTree::new(Some(5), None, BinaryTree::new(Some(10), None, None)),
            ),
        );

        assert_eq!(root.as_ref().unwrap().contains(&10), true);
        assert_eq!(root.as_ref().unwrap().contains(&1), true);
        assert_eq!(root.as_ref().unwrap().contains(&8), true);
        assert_eq!(root.as_ref().unwrap().contains(&0), false);
    }
}
