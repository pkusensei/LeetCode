using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class BSTIterator
{
    readonly Stack<TreeNode> st;

    public BSTIterator(TreeNode root)
    {
        st = [];
        Push(root);
    }

    public int Next()
    {
        var node = st.Pop();
        Push(node.right);
        return node.val;
    }

    public bool HasNext() => st.Count > 0;

    void Push(TreeNode node)
    {
        while (node is not null)
        {
            st.Push(node);
            node = node.left;
        }
    }
}
