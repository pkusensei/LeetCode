using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int TwoCitySchedCost(int[][] costs)
    {
        int n = costs.Length / 2;
        // By selecting x[0], the net impact on budget x[0]-x[1]
        Array.Sort(costs, (a, b) => (a[0] - a[1]).CompareTo(b[0] - b[1]));
        return costs[..n].Select(v => v[0]).Sum() + costs[n..].Select(v => v[1]).Sum();
    }
}

