using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestSubstring(string s, int k)
    {
        return Dfs(s, k);

        static int Dfs(ReadOnlySpan<char> s, int k)
        {
            if (s.Length < k) { return 0; }
            if (k == 1) { return s.Length; }
            Span<int> freq = stackalloc int[26];
            foreach (var c in s)
            {
                freq[c - 'a'] += 1;
            }
            bool fit = true;
            for (int i = 0; i < 26; i++)
            {
                if (0 < freq[i] && freq[i] < k)
                {
                    fit = false;
                    break;
                }
            }
            if (fit) { return s.Length; }
            int res = 0;
            int left = 0;
            for (int right = 0; right < s.Length; right++)
            {
                int c = s[right] - 'a';
                if (0 < freq[c] && freq[c] < k)
                {
                    res = int.Max(res, Dfs(s[left..right], k));
                    left = 1 + right;
                }
            }
            res = int.Max(res, Dfs(s[left..], k));
            return res;
        }
    }
}
