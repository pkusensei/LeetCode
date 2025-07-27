using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxCoins(int[] nums)
    {
        List<int> arr = [1];
        arr.AddRange(nums);
        arr.Add(1);
        int n = arr.Count;
        int[,] memo = new int[n, n];
        for (int i1 = 0; i1 < n; i1++)
        {
            for (int i2 = 0; i2 < n; i2++)
            {
                memo[i1, i2] = -1;
            }
        }
        return Dfs(1, n - 2);

        int Dfs(int left, int right)
        {
            if (left > right) { return 0; }
            if (memo[left, right] > -1) { return memo[left, right]; }
            int res = 0;
            for (int i = left; i <= right; i++)
            {
                int curr = arr[left - 1] * arr[i] * arr[1 + right];
                curr += Dfs(left, i - 1) + Dfs(1 + i, right);
                res = int.Max(res, curr);
            }
            memo[left, right] = res;
            return res;
        }
    }
}