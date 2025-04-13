using System.Numerics;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public long MaximumTripletValue(int[] nums)
    {
        long maxi = 0;
        long delta = 0;
        long res = 0;
        foreach (var num in nums)
        {
            res = Math.Max(res, delta * num);
            delta = Math.Max(delta, maxi - num);
            maxi = Math.Max(maxi, num);
        }
        return res;
    }
}
