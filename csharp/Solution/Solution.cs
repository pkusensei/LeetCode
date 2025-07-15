using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void Flatten(TreeNode root)
    {
        Dfs(root);

        static (TreeNode head, TreeNode tail) Dfs(TreeNode node)
        {
            if (node is null) { return (null, null); }
            var head = node;
            var tail = node;
            var left = Dfs(node.left);
            head.left = null;
            var right = Dfs(node.right);
            if (left.head is not null)
            {
                head.right = left.head;
                tail = left.tail;
            }
            if (right.tail is not null)
            {
                tail.right = right.head;
                tail = right.tail;
            }
            return (head, tail);
        }
    }
}