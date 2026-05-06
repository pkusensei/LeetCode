using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountTriplets(int[] nums)
    {
        Dictionary<int, int> dict = [];
        foreach (var a in nums)
        {
            foreach (var b in nums)
            {
                if (!dict.TryAdd(a & b, 1)) { dict[a & b] += 1; }
            }
        }
        int res = 0;
        foreach (var c in nums)
        {
            foreach (var (k, v) in dict)
            {
                if ((c & k) == 0) { res += v; }
            }
        }
        return res;
    }
}
