using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> BinaryTreePaths(TreeNode root)
    {
        List<string> res = [];
        Dfs(root, []);
        return res;

        void Dfs(TreeNode node, List<int> curr)
        {
            if (node is null) { return; }
            curr.Add(node.val);
            if (node.left is null && node.right is null)
            {
                res.Add(string.Join("->", curr));
            }
            Dfs(node.left, curr);
            Dfs(node.right, curr);
            curr.RemoveAt(curr.Count - 1);
        }
    }
}