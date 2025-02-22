using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode RecoverFromPreorder(string traversal)
    {
        Stack<TreeNode> st = [];
        var idx = 0;
        while (idx < traversal.Length)
        {
            var depth = 0;
            while (idx < traversal.Length && traversal[idx] == '-')
            {
                depth += 1;
                idx += 1;
            }
            int num = 0;
            while (idx < traversal.Length && char.IsAsciiDigit(traversal[idx]))
            {
                num = 10 * num + traversal[idx] - '0';
                idx += 1;
            }
            // Key step: pop nodes until top of stack has depth-1
            while (st.Count > depth) { st.Pop(); }
            var node = new TreeNode(num);
            if (st.TryPeek(out var prev))
            {
                if (prev.left is null) { prev.left = node; }
                else { prev.right = node; }
            }
            st.Push(node);
        }
        return st.LastOrDefault(); // bottom of stack
    }
}

