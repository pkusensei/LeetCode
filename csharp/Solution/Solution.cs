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

    public IList<int> PostorderTraversal(TreeNode root)
    {
        if (root is null) { return []; }
        List<int> res = [];
        Stack<TreeNode> st = [];
        var curr = root;
        TreeNode prev = null;
        while (curr is not null || st.Count > 0)
        {
            if (curr is not null)
            {
                st.Push(curr);
                curr = curr.left;
            }
            else
            {
                var top = st.Peek();
                if (top.right is not null && prev != top.right)
                {
                    curr = top.right;
                }
                else
                {
                    res.Add(top.val);
                    prev = st.Pop();
                }
            }
        }
        return res;
    }
}