using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string LongestSubsequenceRepeatedK(string s, int k)
    {
        Span<int> freq = stackalloc int[26];
        foreach (var c in s)
        {
            freq[c - 'a'] += 1;
        }
        List<char> chs = [];
        for (int i = 25; i >= 0; i -= 1)
        {
            chs.AddRange(Enumerable.Repeat((char)(i + 'a'), freq[i] / k));
        }
        List<char> res = [];
        Backtrack([], 0);
        return new([.. res]);

        void Backtrack(List<char> curr, int mask)
        {
            for (int i = 0; i < chs.Count; i++)
            {
                // 2 <= n < k*8
                // n/k < 8
                if (((mask >> i) & 1) == 1) { continue; }
                curr.Add(chs[i]);
                if (Find(curr, k))
                {
                    if (curr.Count > res.Count) { res = [.. curr]; }
                    Backtrack(curr, mask | (1 << i));
                }
                curr.RemoveAt(curr.Count - 1);
            }
        }

        bool Find(IList<char> curr, int k)
        {
            int i2 = 0;
            for (int i1 = 0; i1 < s.Length && k > 0; i1++)
            {
                if (s[i1] == curr[i2])
                {
                    i2 += 1;
                    if (i2 >= curr.Count)
                    {
                        k -= 1;
                        i2 = 0;
                    }
                }
            }
            return k <= 0;
        }
    }
}
