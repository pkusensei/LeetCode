using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool CheckInclusion(string s1, string s2)
    {
        if (s1.Length > s2.Length) { return false; }
        Span<int> freq = stackalloc int[26];
        foreach (var c in s1)
        {
            freq[c - 'a'] += 1;
        }
        for (int i = 0; i < s2.Length; i++)
        {
            freq[s2[i] - 'a'] -= 1;
            if (i >= s1.Length)
            {
                freq[s2[i - s1.Length] - 'a'] += 1;
            }
            if (i >= s1.Length - 1)
            {
                bool flag = true;
                for (int fi = 0; fi < 26; fi++)
                {
                    if (freq[fi] > 0)
                    {
                        flag = false;
                        break;
                    }
                }
                if (flag) { return true; }
            }
        }
        return false;
    }
}
