using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestSubsequence(string s, int k)
    {
        int val = 1;
        int res = 0;
        for (int i = s.Length - 1; i >= 0; i -= 1)
        {
            if (s[i] == '0' || val <= k)
            {
                k -= val * (s[i] - '0');
                res += 1;
            }
            if (val <= k) { val *= 2; }
        }
        return res;
    }
}
