using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> PathInZigZagTree(int label)
    {
        if (label <= 1) { return [1]; }
        int pow = Math.ILogB(label);
        int prev_opposite = label / 2;
        int prev_start = (int)Math.Pow(2, pow - 1);
        int prev_end = (int)Math.Pow(2, pow) - 1;
        int diff = prev_opposite - prev_start;
        int distance = prev_end - prev_start;
        int v = prev_start + (distance - diff);
        var res = PathInZigZagTree(v);
        res.Add(label);
        return res;
    }
}
