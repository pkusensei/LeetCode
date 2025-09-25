using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int StrangePrinter(string s)
    {
        List<int> nums = [];
        int left = 0;
        for (int right = left; right < s.Length; right += 1)
        {
            if (s[right] == s[left]) { continue; }
            nums.Add(s[left] - 'a');
            left = right;
        }
        nums.Add(s[left] - 'a');
        int n = nums.Count;
        int[,] memo = new int[n, n];
        return Dfs(0, n - 1);

        int Dfs(int left, int right)
        {
            if (left > right) { return 0; }
            if (memo[left, right] > 0) { return memo[left, right]; }
            int res = 1 + Dfs(1 + left, right);
            for (int i = 1 + left; i <= right; i++)
            {
                if (nums[i] == nums[left])
                {
                    // abac => ab + c
                    res = int.Min(res, Dfs(left, i - 1) + Dfs(1 + i, right));
                }
            }
            memo[left, right] = res;
            return res;
        }
    }
}