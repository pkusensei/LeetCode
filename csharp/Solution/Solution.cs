using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Flipgame(int[] fronts, int[] backs)
    {
        var ban = fronts.Zip(backs).Where(v => v.First == v.Second)
            .Select(v => v.First).ToFrozenSet();
        int res = int.MaxValue;
        foreach (var item in fronts.Concat(backs))
        {
            if (!ban.Contains(item)) { res = int.Min(res, item); }
        }
        return res == int.MaxValue ? 0 : res;
    }
}
