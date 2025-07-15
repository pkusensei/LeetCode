using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IsSymmetric(TreeNode root)
    {
        if (root is null) { return true; }
        return Dfs(root.left, root.right);

        static bool Dfs(TreeNode left, TreeNode right)
        {
            if (left is null && right is null) { return true; }
            if (left is null ^ right is null) { return false; }
            return left.val == right.val
                   && Dfs(left.left, right.right)
                   && Dfs(left.right, right.left);
        }
    }

    public bool WithStack(TreeNode root)
    {
        if (root is null) { return true; }
        Stack<TreeNode> st = [];
        st.Push(root.left);
        st.Push(root.right);
        while (st.Count > 1)
        {
            var n1 = st.Pop();
            var n2 = st.Pop();
            if (n1 is null && n2 is null) { continue; }
            if (n1 is null ^ n2 is null) { return false; }
            if (n1.val != n2.val) { return false; }
            st.Push(n1.left);
            st.Push(n2.right);
            st.Push(n1.right);
            st.Push(n2.left);
        }
        return true;
    }
}