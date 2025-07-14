using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> InorderTraversal(TreeNode root)
    {
        List<int> res = [];
        if (root is null) { return res; }
        Stack<TreeNode> st = [];
        var curr = root;
        while (curr is not null || st.Count > 0)
        {
            while (curr is not null)
            {
                st.Push(curr);
                curr = curr.left;
            }
            curr = st.Pop();
            res.Add(curr.val);
            curr = curr.right;
        }
        return res;
    }
}