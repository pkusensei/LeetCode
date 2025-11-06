using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> PartitionLabels(string s)
    {
        Span<int> last = stackalloc int[26];
        for (int i = 0; i < s.Length; i++)
        {
            last[s[i] - 'a'] = i;
        }
        List<int> res = [];
        int left = 0;
        int end = 0;
        for (int right = 0; right < s.Length; right++)
        {
            int curr = s[right] - 'a';
            end = int.Max(end, last[curr]);
            if (end == right)
            {
                res.Add(right - left + 1);
                left = 1 + right;
            }
        }
        return res;
    }
}