using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int GetMoneyAmount(int n)
    {
        int[,] memo = new int[1 + n, 1 + n];
        for (int i = 0; i <= n; i++)
        {
            for (int j = 0; j <= n; j++)
            {
                memo[i, j] = -1;
            }
        }
        return Dfs(1, n);

        int Dfs(int left, int right)
        {
            if (left >= right) { return 0; } // Found!
            if (memo[left, right] > -1) { return memo[left, right]; }
            int res = int.MaxValue;
            for (int i = left; i <= right; i++)
            {
                // Goes on either branch, pick the big one
                int curr = i + int.Max(Dfs(left, i - 1), Dfs(1 + i, right));
                res = int.Min(res, curr);
            }
            memo[left, right] = res;
            return res;
        }
    }
}