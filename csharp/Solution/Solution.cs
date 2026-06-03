using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode BstFromPreorder(int[] preorder)
    {
        TreeNode root = new(preorder[0]);
        Stack<TreeNode> st = [];
        st.Push(root);
        foreach (var num in preorder[1..])
        {
            TreeNode node = new(num);
            TreeNode temp = null;
            while (st.TryPeek(out var top) && top.val < num)
            {
                temp = st.Pop();
            }
            if (temp is not null)
            {
                temp.right = node;
            }
            else if (st.TryPeek(out var top_))
            {
                top_.left = node;
            }
            st.Push(node);
        }
        return root;
    }

    public TreeNode WithDfs(int[] preorder)
    {
        return Dfs(preorder);

        static TreeNode Dfs(ReadOnlySpan<int> preorder)
        {
            if (preorder.IsEmpty) { return null; }
            int val = preorder[0];
            int i = 0;
            for (; i < preorder.Length && preorder[i] <= val; i += 1)
            { }
            TreeNode left = Dfs(preorder[1..i]);
            TreeNode right = Dfs(preorder[i..]);
            return new(val, left, right);
        }
    }
}

