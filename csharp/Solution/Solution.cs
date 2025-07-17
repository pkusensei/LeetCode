using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> PreorderTraversal(TreeNode root)
    {
        if (root is null) { return []; }
        List<int> res = [];
        Stack<TreeNode> st = [];
        st.Push(root);
        while (st.TryPop(out var node))
        {
            if (node is not null)
            {
                res.Add(node.val);
                st.Push(node.right);
                st.Push(node.left);
            }
        }
        return res;
    }
}