using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxProfitAssignment(int[] difficulty, int[] profit, int[] worker)
    {
        (int d, int p)[] arr = [.. difficulty.Zip(profit).Order().Select(v => (v.First, v.Second))];
        int max_p = 0;
        int res = 0;
        int i = 0;
        Array.Sort(worker);
        foreach (var item in worker)
        {
            while (i < arr.Length && arr[i].d <= item)
            {
                max_p = int.Max(max_p, arr[i].p);
                i += 1;
            }
            res += max_p;
        }
        return res;
    }
}
