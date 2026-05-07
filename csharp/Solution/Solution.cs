using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string SmallestFromLeaf(TreeNode root)
    {
        StringBuilder sb = new();
        string res = null;
        Dfs(root);
        return res;

        void Dfs(TreeNode node)
        {
            if (node is null) { return; }
            sb.Insert(0, (char)(node.val + 'a'));
            if (node.left is null && node.right is null)
            {
                string s = sb.ToString();
                if (string.IsNullOrEmpty(res) || string.Compare(s, res) < 0)
                {
                    res = s;
                }
            }
            Dfs(node.left);
            Dfs(node.right);
            sb.Remove(0, 1);
        }
    }
}
