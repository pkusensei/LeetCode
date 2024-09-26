using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    List<int> res = [];
    int currNum = 0;
    int currStreak = 0;
    int maxStreak = 0;

    public int[] FindMode(TreeNode root)
    {
        Dfs(root);
        return [.. res];
    }

    void Dfs(TreeNode node)
    {
        if (node is null) { return; }
        Dfs(node.left);
        if (currNum == node.val)
        {
            currStreak += 1;
        }
        else
        {
            currStreak = 1;
            currNum = node.val;
        }
        if (currStreak > maxStreak)
        {
            res.Clear();
            maxStreak = currStreak;
        }
        if (currStreak == maxStreak) { res.Add(node.val); }
        Dfs(node.right);
    }
}