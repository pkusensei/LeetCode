using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindLongestChain(int[][] pairs)
    {
        Array.Sort(pairs, (a, b) => a[1].CompareTo(b[1]));
        int end = -1001;
        int res = 0;
        foreach (var item in pairs)
        {
            if (end < item[0])
            {
                res += 1;
                end = item[1];
            }
        }
        return res;
    }
}