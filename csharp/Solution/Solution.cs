using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxCount(int m, int n, int[][] ops)
    {
        if (ops is null || ops.Length == 0) { return m * n; }
        return ops.Min(v => v[0]) * ops.Min(v => v[1]);
    }
}