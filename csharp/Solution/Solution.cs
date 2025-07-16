using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CanCompleteCircuit(int[] gas, int[] cost)
    {
        if (gas.Sum() < cost.Sum()) { return -1; }
        int res = 0;
        int curr = 0;
        for (int i = 0; i < gas.Length; i++)
        {
            curr += gas[i] - cost[i];
            if (curr < 0)
            {
                curr = 0;
                res = 1 + i;
            }
        }
        return res;
    }
}