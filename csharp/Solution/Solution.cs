using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> PowerfulIntegers(int x, int y, int bound)
    {
        if (bound < 2) { return []; }
        if (x == 1 && y == 1) { return [2]; }
        if (x == 1 || y == 1)
        {
            int num = int.Max(x, y);
            List<int> v = [];
            for (int p = 1; 1 + p <= bound; p *= num)
            {
                v.Add(1 + p);
            }
            return v;
        }
        double px = Math.Log(bound, x);
        double py = Math.Log(bound, y);
        HashSet<int> res = [];
        for (int ix = 0; ix <= px; ix++)
        {
            for (int iy = 0; iy <= py; iy++)
            {
                int v = (int)(Math.Pow(x, ix) + Math.Pow(y, iy));
                if (v <= bound) { res.Add(v); }
                else { break; }
            }
        }
        return [.. res];
    }
}
