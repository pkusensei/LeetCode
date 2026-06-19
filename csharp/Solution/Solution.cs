using System.Collections.Frozen;
using System.Runtime.InteropServices;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LongestStrChain(string[] words)
    {
        Array.Sort(words, (a, b) => a.Length.CompareTo(b.Length));
        Dictionary<string, int> dp = new(words.Length);
        int res = 1;
        foreach (var item in words)
        {
            int curr = 1;
            for (int i = 0; i < item.Length; i++)
            {
                string s = item[..i] + item[(1 + i)..];
                if (dp.TryGetValue(s, out var prev))
                {
                    curr = int.Max(curr, 1 + prev);
                }
            }
            res = int.Max(res, curr);
            dp[item] = curr;
        }
        return res;
    }
}
