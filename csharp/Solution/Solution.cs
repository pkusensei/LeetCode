using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] SumZero(int n)
    {
        List<int> res = new(n);
        if ((n & 1) == 1) { res.Add(0); }
        for (int i = 0; i < n / 2; i++)
        {
            res.AddRange([-(1 + i), 1 + i]);
        }
        return [.. res];
    }
}
