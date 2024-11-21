using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public TreeNode RecoverFromPreorder(string s)
    {
        var stack = new Stack<TreeNode>();
        var idx = 0;
        while (idx < s.Length)
        {
            var depth = 0;
            while (idx < s.Length && s[idx] == '-')
            {
                depth += 1;
                idx += 1;
            }
            var sb = new StringBuilder();
            while (idx < s.Length && s[idx] != '-')
            {
                sb.Append(s[idx]);
                idx += 1;
            }
            var num = int.Parse(sb.ToString());
            TreeNode node = new(num);
            while (stack.Count > depth)
            {
                stack.Pop();
            }
            if (stack.TryPeek(out var parent))
            {
                if (parent.left is null) { parent.left = node; }
                else { parent.right = node; }
            }
            stack.Push(node);
        }
        return stack.Last();
    }
}
