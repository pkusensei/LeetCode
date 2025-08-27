using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindRadius(int[] houses, int[] heaters)
    {
        Array.Sort(heaters);
        int n = heaters.Length;
        int res = 0;
        foreach (var house in houses)
        {
            int idx = Array.BinarySearch(heaters, house);
            if (idx < 0)
            {
                idx = ~idx;
                if (idx == 0) { res = int.Max(res, heaters[0] - house); }
                else if (idx == n) { res = int.Max(res, house - heaters[n - 1]); }
                else { res = int.Max(res, int.Min(house - heaters[idx - 1], heaters[idx] - house)); }
            }
        }
        return res;
    }
}