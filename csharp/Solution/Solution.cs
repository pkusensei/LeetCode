using System.Numerics;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int NumberOfArrays(int[] differences, int lower, int upper)
    {
        long min = 0;
        long max = 0;
        long curr = 0;
        foreach (var d in differences)
        {
            curr += d;
            min = Math.Min(min, curr);
            max = Math.Max(max, curr);
        }
        return Math.Max((upper - lower + 1) - (int)(max - min), 0);
    }
}
