using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<TreeNode> AllPossibleFBT(int n)
    {
        if ((n & 1) == 0) { return []; }
        return Dfs(n);

        List<TreeNode> Dfs(int n)
        {
            if (n == 1) { return [new()]; }
            List<TreeNode> res = [];
            for (int i = 1; i < n; i += 2)
            {
                var left = Dfs(i);
                var right = Dfs(n - i - 1);
                foreach (var a in left)
                {
                    foreach (var b in right)
                    {
                        res.Add(new(0, a, b));
                    }
                }
            }
            return res;
        }
    }
}