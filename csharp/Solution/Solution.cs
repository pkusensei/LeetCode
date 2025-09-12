using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxDistance(IList<IList<int>> arrays)
    {
        int min = arrays[0][0];
        int max = arrays[0].Last();
        int res = 0;
        foreach (var row in arrays.Skip(1))
        {
            int curr_min = row[0];
            int curr_max = row.Last();
            res = int.Max(res, int.Max(int.Abs(curr_max - min), int.Abs(max - curr_min)));
            min = int.Min(min, curr_min);
            max = int.Max(max, curr_max);
        }
        return res;
    }
}