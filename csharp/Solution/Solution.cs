using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindMinMoves(int[] machines)
    {
        int n = machines.Length;
        int sum = machines.Sum();
        if (sum % n > 0) { return -1; }
        int ave = sum / n;
        int prefix = 0;
        int res = 0;
        for (int i = 0; i < n; i++)
        {
            res = int.Max(res, machines[i] - ave);
            prefix += machines[i] - ave;
            res = int.Max(res, prefix);
        }
        int suffix = 0;
        for (int i = n - 1; i >= 0; i -= 1)
        {
            suffix += machines[i] - ave;
            res = int.Max(res, suffix);
        }
        return res;
    }
}