using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int GetMaxRepetitions(string s1, int n1, string s2, int n2)
    {
        int len1 = s1.Length;
        int len2 = s2.Length;
        int i1 = 0;
        int i2 = 0;
        Dictionary<(int, int), (int, int)> seen = [];
        while (i1 < len1 * n1)
        {
            if (s1[i1 % len1] == s2[i2 % len2])
            {
                if (seen.TryGetValue((i1 % len1, i2 % len2), out var prev))
                {
                    (int prev1, int prev2) = prev;
                    int p1 = i1 - prev1;
                    int count = (len1 * n1 - i1) / p1;
                    i1 += count * p1;
                    i2 += count * (i2 - prev2);
                    if (i1 >= len1 * n1) { break; }
                }
                else
                {
                    seen.Add((i1 % len1, i2 % len2), (i1, i2));
                }
                i2 += 1;
            }
            i1 += 1;
        }
        return i2 / len2 / n2;
    }
}