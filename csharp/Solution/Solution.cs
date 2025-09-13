using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxFreqSum(string s)
    {
        Span<int> freq = stackalloc int[26];
        foreach (var ch in s)
        {
            freq[ch - 'a'] += 1;
        }
        int v = 0;
        int c = 0;
        for (int i = 0; i < 26; i += 1)
        {
            int curr = freq[i];
            if ("aeiou".Contains((char)(i + 'a'))) { v = int.Max(v, curr); }
            else { c = int.Max(c, curr); }
        }
        return v + c;
    }
}