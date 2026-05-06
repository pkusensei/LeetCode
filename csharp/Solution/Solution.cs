using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] SumEvenAfterQueries(int[] nums, int[][] queries)
    {
        List<int> res = new(queries.Length);
        int sum = nums.Where(v => (v & 1) == 0).Sum();
        foreach (var q in queries)
        {
            int prev = nums[q[1]];
            if ((prev & 1) == 0) { sum -= prev; }
            nums[q[1]] += q[0];
            if ((nums[q[1]] & 1) == 0) { sum += nums[q[1]]; }
            res.Add(sum);
        }
        return [.. res];
    }
}
