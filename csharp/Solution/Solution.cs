using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public string GetDirections(TreeNode root, int startValue, int destValue)
    {
        var sb1 = Dfs(root, startValue);
        var sb2 = Dfs(root, destValue);
        while (sb1.Length > 0 && sb2.Length > 0 && sb1[^1] == sb2[^1])
        {
            sb1.Remove(sb1.Length - 1, 1);
            sb2.Remove(sb2.Length - 1, 1);
        }
        StringBuilder sb = new();
        for (int _ = 0; _ < sb1.Length; _++)
        {
            sb.Append('U');
        }
        for (int i = sb2.Length - 1; i >= 0; i -= 1)
        {
            sb.Append(sb2[i]);
        }
        return sb.ToString();

        static StringBuilder Dfs(TreeNode node, int target)
        {
            if (node is null) { return null; }
            if (node.val == target) { return new(); }
            var left = Dfs(node.left, target);
            var right = Dfs(node.right, target);
            if (left is not null) { left.Append('L'); }
            else { right?.Append('R'); }
            return left ?? right;
        }
    }
}
