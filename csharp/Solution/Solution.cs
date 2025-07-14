using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<TreeNode> GenerateTrees(int n)
    {
        return Backtrack(1, n);

        static List<TreeNode> Backtrack(int left, int right)
        {
            if (left > right) { return [null]; }
            List<TreeNode> res = [];
            for (int i = left; i <= right; i++)
            {
                var left_trees = Backtrack(left, i - 1);
                var right_trees = Backtrack(1 + i, right);
                foreach (var a in left_trees)
                {
                    foreach (var b in right_trees)
                    {
                        TreeNode node = new(i, a, b);
                        res.Add(node);
                    }
                }
            }
            return res;
        }
    }
}