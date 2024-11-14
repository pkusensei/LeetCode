using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<int> FlipMatchVoyage(TreeNode root, int[] voyage)
    {
        List<int> res = [];
        int idx = 0;
        Dfs(root);
        if (res.Count > 0 && res[0] == -1)
        {
            return [-1];
        }
        return res;

        void Dfs(TreeNode node)
        {
            if (node is null) { return; }
            if (node.val != voyage[idx])
            {
                res = [-1];
                return;
            }
            idx += 1;
            if (idx < voyage.Length && node.left is not null && node.left.val != voyage[idx])
            {
                res.Add(node.val);
                Dfs(node.right);
                Dfs(node.left);
            }
            else
            {
                Dfs(node.left);
                Dfs(node.right);
            }
        }
    }
}
