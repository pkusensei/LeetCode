using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> GetSkyline(int[][] buildings)
    {
        List<(int x, int y)> arr = [];
        foreach (var b in buildings)
        {
            arr.Add((b[0], -b[2]));
            arr.Add((b[1], b[2]));
        }
        arr.Sort();
        SortedList<int, int> dict = [];
        List<IList<int>> res = [];
        int prev = 0;
        foreach (var (x, y) in arr)
        {
            if (y < 0 && !dict.TryAdd(-y, 1)) { dict[-y] += 1; }
            else if (y > 0)
            {
                dict[y] -= 1;
                if (dict[y] == 0) { dict.Remove(y); }
            }
            var height = dict.Keys.LastOrDefault();
            if (height != prev) { res.Add([x, height]); }
            prev = height;
        }
        return res;
    }
}