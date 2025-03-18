using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MinGroups(int[][] intervals)
    {
        SortedList<int, int> map = [];
        foreach (var item in intervals)
        {
            if (!map.TryAdd(item[0], 1)) { map[item[0]] += 1; }
            if (!map.TryAdd(item[1] + 1, -1)) { map[item[1] + 1] -= 1; }
        }
        int curr = 0;
        int res = 0;
        foreach (var val in map.Values)
        {
            curr += val;
            res = Math.Max(res, curr);
        }
        return res;
    }
}
