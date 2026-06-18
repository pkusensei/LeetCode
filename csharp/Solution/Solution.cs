using System.Collections.Frozen;
using System.Runtime.InteropServices;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string LongestDupSubstring(string s)
    {
        const long BASE = 127;
        const long M = 1_000_000_007;

        int n = s.Length;
        List<long> prefix = new(1 + n) { 0 };
        List<long> pows = new(1 + n) { 1 };
        foreach (var c in s)
        {
            prefix.Add((prefix[^1] * BASE + c - 'a') % M);
            pows.Add(pows[^1] * BASE % M);
        }
        int left = 0;
        int right = n;
        ReadOnlySpan<char> res = "";
        while (left < right)
        {
            int mid = left + (1 + right - left) / 2;
            var curr = Find(mid, pows[mid]);
            if (curr.Length > res.Length)
            {
                res = curr;
                left = mid;
            }
            else
            {
                right = mid - 1;
            }
        }
        return new(res);

        ReadOnlySpan<char> Find(int len, long pow)
        {
            Dictionary<long, int> dict = [];
            for (int left = 0; left <= n - len; left++)
            {
                long hash = (prefix[left + len] - prefix[left] * pow % M + M) % M;
                if (dict.TryGetValue(hash, out int prev)
                && s.Substring(left, len) == s.Substring(prev, len))
                {
                    return s.AsSpan(left..(left + len));
                }
                dict[hash] = left;
            }
            return "";
        }
    }
}
