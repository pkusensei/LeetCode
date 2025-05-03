using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinDominoRotations(int[] tops, int[] bottoms)
    {
        if (tops.Length != bottoms.Length) { return -1; }
        int a = Solve(tops[0]);
        int b = Solve(bottoms[0]);
        return (a == -1, b == -1) switch
        {
            (true, true) => -1,
            (true, false) => b,
            (false, true) => a,
            _ => Math.Min(a, b)
        };

        int Solve(int target)
        {
            int count1 = 0;
            int count2 = 0;
            foreach (var (a, b) in tops.Zip(bottoms))
            {
                if (target != a && target != b) { return -1; }
                else if (target == a && target != b) { count2 += 1; }
                else if (target != a && target == b) { count1 += 1; }
            }
            return Math.Min(count1, count2);
        }
    }
}
