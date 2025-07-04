using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string LongestPalindrome(string s)
    {
        int n = s.Length;
        bool[,] dp = new bool[n, n];
        ReadOnlySpan<char> res = s.AsSpan(0, 1);
        for (int i = 0; i < n; i++)
        {
            dp[i, i] = true;
            if (i < n - 1 && s[i] == s[1 + i])
            {
                dp[i, 1 + i] = true;
                if (res.Length < 2) { res = s.AsSpan(i, 2); }
            }
        }
        for (int mid = 0; mid < n; mid++)
        {
            int left = mid - 1;
            int right = 1 + mid;
            while (0 <= left && right < n && s[left] == s[right] && dp[left + 1, right - 1])
            {
                dp[left, right] = true;
                if (res.Length < right + 1 - left) { res = s.AsSpan(left, right + 1 - left); }
                left -= 1;
                right += 1;
            }
            left = mid - 1;
            right = 2 + mid;
            while (0 <= left && right < n && s[left] == s[right] && dp[left + 1, right - 1])
            {
                dp[left, right] = true;
                if (res.Length < right + 1 - left) { res = s.AsSpan(left, right + 1 - left); }
                left -= 1;
                right += 1;
            }
        }
        return new(res);
    }
}
