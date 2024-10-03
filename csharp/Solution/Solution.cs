// using Solution.LList;
using System.Text;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public string Tree2str(TreeNode root)
    {
        var sb = new StringBuilder();
        if (root is null) { return sb.ToString(); }
        Dfs(root, sb);
        return sb.ToString();
    }

    static void Dfs(TreeNode node, StringBuilder sb)
    {
        sb.Append(node.val);
        switch (node.left is null, node.right is null)
        {
            case (true, true): return;
            case (true, false):
                sb.Append("()(");
                Dfs(node.right, sb);
                sb.Append(')');
                return;
            case (false, true):
                sb.Append('(');
                Dfs(node.left, sb);
                sb.Append(')');
                return;
            default:
                sb.Append('(');
                Dfs(node.left, sb);
                sb.Append(')');
                sb.Append('(');
                Dfs(node.right, sb);
                sb.Append(')');
                return;
        }
    }
}
