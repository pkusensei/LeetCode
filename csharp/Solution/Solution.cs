using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string Tree2str(TreeNode root)
    {
        List<string> res = [];
        Dfs(root);
        return string.Join("", res);

        void Dfs(TreeNode node)
        {
            if (node is null) { return; }
            res.Add(node.val.ToString());
            res.Add("(");
            Dfs(node.left);
            res.Add(")");
            res.Add("(");
            Dfs(node.right);
            res.Add(")");
            for (int _ = 0; _ < 2; _++)
            {
                if (res[^2] == "(" && res[^1] == ")")
                {
                    res.RemoveRange(res.Count - 2, 2);
                }
            }
        }
    }
}