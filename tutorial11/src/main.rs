fn main() {
    let b_int1 = Box::new(10);
    println!("{}", b_int1);

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(g_key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key: g_key,
            }
        }

        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }

        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    let node1 = TreeNode::new(1).left(TreeNode::new(40));
    let node1 = node1.right(TreeNode::new(40));
    println!("{}", node1.key)
}
