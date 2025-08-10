using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Solution()
    {
        Valid = new int[32][];
        for (int p = 0; p < 32; p++)
        {
            Valid[p] = Count((int)Math.Pow(2, p));
        }
    }

    public bool ReorderedPowerOf2(int n)
    {
        var curr = Count(n);
        return Valid.Any(v => v.SequenceEqual(curr));
    }

    int[][] Valid { get; }

    private static int[] Count(int num)
    {
        string s = num.ToString();
        int[] curr = new int[10];
        foreach (var c in s)
        {
            curr[c - '0'] += 1;
        }
        return curr;
    }
}