using Solution.LList;
using Solution.Tree;

namespace Solution;

public class CBTInserter
{
    TreeNode root;

    public CBTInserter(TreeNode root)
    {
        this.root = root;
    }

    public int Insert(int val)
    {
        var queue = new Queue<TreeNode>();
        queue.Enqueue(root);
        while (queue.TryDequeue(out var node))
        {
            if (node.left is null) { node.left = new(val); return node.val; }
            if (node.right is null) { node.right = new(val); return node.val; }
            queue.Enqueue(node.left);
            queue.Enqueue(node.right);
        }
        return -1;
    }

    public TreeNode Get_root()
    {
        return root;
    }
}

public class Solution
{

}
