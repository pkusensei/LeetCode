using System.Collections.Immutable;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<int> GetAllElements(TreeNode root1, TreeNode root2)
    {
        Stack<TreeNode> st1 = [];
        Stack<TreeNode> st2 = [];
        PushLeft(st1, root1);
        PushLeft(st2, root2);
        List<int> res = [];
        while (st1.Count > 0 || st2.Count > 0)
        {
            var stack = (st1.TryPeek(out var v1), st2.TryPeek(out var v2)) switch
            {
                (false, true) => st2,
                (true, false) => st1,
                _ => v1.val < v2.val ? st1 : st2,
            };
            var curr = stack.Pop();
            res.Add(curr.val);
            PushLeft(stack, curr.right);
        }
        return res;

        static void PushLeft(Stack<TreeNode> st, TreeNode node)
        {
            while (node is not null)
            {
                st.Push(node);
                node = node.left;
            }
        }
    }
}