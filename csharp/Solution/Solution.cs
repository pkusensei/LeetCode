using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LeastOpsExpressTarget(int x, int target)
    {
        Dictionary<long, int> memo = [];
        return Dfs(x, target);

        int Dfs(int x, long target)
        {
            if (target < x) { return (int)long.Min(2 * target - 1, 2 * (x - target)); }
            if (memo.TryGetValue(target, out int res)) { return res; }
            int p = (int)Math.Log(target, x);
            long big = (long)Math.Pow(x, 1 + p);
            if (big == target)
            {
                res = p;
            }
            else
            {
                res = p + Dfs(x, target - big / x);
                if (big - target < target)
                {
                    res = int.Min(res, 1 + p + Dfs(x, big - target));
                }
            }
            memo.Add(target, res);
            return res;
        }
    }
}
