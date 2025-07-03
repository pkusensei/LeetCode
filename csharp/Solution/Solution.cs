using System.Security.Principal;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LengthOfLongestSubstring(string s)
    {
        Span<int> last = stackalloc int[128];
        last.Fill(-1);
        int res = 0;
        int left = 0;
        for (int right = 0; right < s.Length; right++)
        {
            int last_ = last[s[right]];
            last[s[right]] = right;
            left = Math.Max(left, last_ + 1);
            res = Math.Max(res, right + 1 - left);
        }
        return res;
    }
}
