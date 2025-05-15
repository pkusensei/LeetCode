using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> GetLongestSubsequence(string[] words, int[] groups)
    {
        List<string> res = [];
        int prev = -1;
        foreach (var (s, num) in words.Zip(groups))
        {
            if (num != prev)
            {
                prev = num;
                res.Add(s);
            }
        }
        return res;
    }
}
