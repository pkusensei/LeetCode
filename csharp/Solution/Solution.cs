using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumPushes(string word)
    {
        Span<int> freq = stackalloc int[26];
        foreach (var c in word)
        {
            freq[c - 'a'] += 1;
        }
        freq.Sort((x, y) => y - x);
        int res = 0;
        for (int i = 0; i < 26; i++)
        {
            res += freq[i] * (1 + i / 8);
        }
        return res;
    }
}
