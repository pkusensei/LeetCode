using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinScoreTriangulation(int[] values)
    {
        int n = values.Length;
        int[,] memo = new int[n, n];
        return Dfs(0, n - 1);

        int Dfs(int left, int right)
        {
            if (right - left + 1 < 3) { return 0; }
            if (memo[left, right] > 0) { return memo[left, right]; }
            int res = int.MaxValue;
            for (int mid = 1 + left; mid < right; mid++)
            {
                int curr = values[left] * values[right] * values[mid];
                res = int.Min(res, curr + Dfs(left, mid) + Dfs(mid, right));
            }
            memo[left, right] = res;
            return res;
        }
    }
}