using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaximumCandies(int[] candies, long k)
    {
        var sum = candies.Select(v => (long)v).Sum();
        if (sum < k) { return 0; }
        int left = 1;
        int right = candies.Max();
        while (left < right)
        {
            var mid = (left + right + 1) / 2;
            if (candies.Select(v => (long)(v / mid)).Sum() >= k) { left = mid; }
            else { right = mid - 1; }
        }
        return left;
    }
}
