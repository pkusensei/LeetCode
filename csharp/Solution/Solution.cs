using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountBinarySubstrings(string s)
    {
        int res = 0;
        int prev = 0;
        int streak = 1;
        for (int i = 1; i < s.Length; i++)
        {
            if (s[i] == s[i - 1]) { streak += 1; }
            else
            {
                res += int.Min(prev, streak);
                prev = streak;
                streak = 1;
            }
        }
        return res + int.Min(prev, streak);
    }
}