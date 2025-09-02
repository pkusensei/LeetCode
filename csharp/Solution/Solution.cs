using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] FindMode(TreeNode root)
    {
        List<int> res = [];
        int curr_num = 100_000;
        int curr_streak = 0;
        int max_streak = 0;
        Dfs(root);
        return [.. res];

        void Dfs(TreeNode node)
        {
            if (node is null) { return; }
            Dfs(node.left);
            if (curr_num == node.val) { curr_streak += 1; }
            else
            {
                curr_streak = 1;
                curr_num = node.val;
            }
            if (curr_streak > max_streak)
            {
                res.Clear();
                max_streak = curr_streak;
            }
            if (curr_streak == max_streak) { res.Add(node.val); }
            Dfs(node.right);
        }
    }
}