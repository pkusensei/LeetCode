using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxSumTwoNoOverlap(int[] nums, int firstLen, int secondLen)
    {
        return int.Max(Solve(firstLen, secondLen), Solve(secondLen, firstLen));

        int Solve(int len1, int len2)
        {
            int n = nums.Length;
            List<int> prefix = new(n);
            int running = 0;
            for (int i = 0; i < n; i++)
            {
                running += nums[i];
                if (i >= len1) { running -= nums[i - len1]; }
                prefix.Add(running);
                if (i > 0) { prefix[i] = int.Max(prefix[i], prefix[i - 1]); }
            }
            running = 0;
            List<int> suffix = new(n);
            for (int i = n - 1; i >= 0; i -= 1)
            {
                running += nums[i];
                if (i + len2 < n) { running -= nums[i + len2]; }
                suffix.Add(running);
                if (suffix.Count >= 2) { suffix[^1] = int.Max(suffix[^1], suffix[^2]); }
            }
            suffix.Reverse();
            int res = 0;
            for (int i = len1 - 1; i + 1 + len2 <= n; i++)
            {
                int val = prefix[i] + suffix[1 + i];
                res = int.Max(res, val);
            }
            return res;
        }
    }
}

