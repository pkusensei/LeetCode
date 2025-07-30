using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool IncreasingTriplet(int[] nums)
    {
        if (nums.Length < 3) { return false; }
        int n1 = int.MaxValue;
        int n2 = int.MaxValue;
        foreach (var num in nums)
        {
            if (num <= n1) { n1 = num; }
            else if (num <= n2) { n2 = num; }
            else { return true; }
        }
        return false;
    }
}
