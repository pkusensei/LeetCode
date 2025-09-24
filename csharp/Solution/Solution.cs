using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<TreeNode> FindDuplicateSubtrees(TreeNode root)
    {
        Dictionary<string, TreeNode> seen = [];
        Dfs(root);
        return [.. seen.Values.Where(s => s is not null)];

        string Dfs(TreeNode node)
        {
            if (node is null) { return "#"; }
            StringBuilder sb = new();
            sb.Append(node.val);
            sb.Append(',');
            string left = Dfs(node.left);
            string right = Dfs(node.right);
            sb.Append(left);
            sb.Append(right);
            string s = sb.ToString();
            if (seen.TryGetValue(s, out var v))
            {
                // second time, record this node
                if (v is null) { seen[s] = node; }
            }
            else
            {
                // first time, mark this string
                seen.Add(s, null);
            }
            return s;
        }
    }
}