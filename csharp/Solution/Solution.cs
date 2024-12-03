using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxLevelSum(TreeNode root)
    {
        List<TreeNode> list = [root];
        var level = 1;
        var res = 0;
        var sum = int.MinValue;
        while (list.Count > 0)
        {
            var curr = list.Select(n => n.val).Sum();
            if (curr > sum)
            {
                sum = curr;
                res = level;
            }
            list = list.SelectMany(n => new[] { n.left, n.right }).Where(n => n is not null).ToList();
            level += 1;
        }
        return res;
    }
}
