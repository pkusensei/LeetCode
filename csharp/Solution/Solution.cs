using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinCut(string s)
    {
        if (IsPalindrome(s)) { return 0; }
        int n = s.Length;
        int[] dp = [.. Enumerable.Range(0, n)];
        for (int mid = 0; mid < n; mid++)
        {
            for (int left = mid, right = mid; 0 <= left && right < n && s[left] == s[right]; left -= 1, right += 1)
            {
                int curr = left == 0 ? 0 : 1 + dp[left - 1];
                dp[right] = int.Min(dp[right], curr);
            }
            for (int left = mid, right = 1 + mid; 0 <= left && right < n && s[left] == s[right]; left -= 1, right += 1)
            {
                int curr = left == 0 ? 0 : 1 + dp[left - 1];
                dp[right] = int.Min(dp[right], curr);
            }
        }
        return dp.Last();

        static bool IsPalindrome(ReadOnlySpan<char> s)
        {
            if (s.Length < 2) { return true; }
            for (int i = 0; i < s.Length; i += 1)
            {
                if (s[i] != s[s.Length - 1 - i]) { return false; }
            }
            return true;
        }
    }
}