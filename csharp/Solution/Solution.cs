using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MakeArrayIncreasing(int[] arr1, int[] arr2)
    {
        Array.Sort(arr2);
        int n1 = arr1.Length;
        int n2 = arr2.Length;
        Dictionary<int, int> dp = new() { [-1] = 0 };
        foreach (var item in arr1)
        {
            if (dp.Count == 0) { break; }
            Dictionary<int, int> curr = [];
            foreach (var (prev, val) in dp)
            {
                if (prev < item)
                {
                    int v = curr.GetValueOrDefault(item, val);
                    curr[item] = int.Min(v, val);
                }
                int i2 = arr2.BinarySearch(1 + prev);
                if (i2 < 0) { i2 = ~i2; }
                if (i2 < n2)
                {
                    int v = curr.GetValueOrDefault(arr2[i2], 1 + val);
                    curr[arr2[i2]] = int.Min(v, 1 + val);
                }
            }
            dp = curr;
        }
        return dp.Count == 0 ? -1 : dp.Values.Min();
        int res = Dfs(0, -1);
        return res < (int.MaxValue >> 1) ? res : -1;

        int Dfs(int idx, int prev)
        {
            if (idx >= n1) { return 0; }
            int res = int.MaxValue >> 1;
            if (prev < arr1[idx]) { res = Dfs(1 + idx, arr1[idx]); }
            int i2 = arr2.BinarySearch(1 + prev);
            if (i2 < 0) { i2 = ~i2; }
            if (i2 < n2)
            {
                int curr = 1 + Dfs(1 + idx, arr2[i2]);
                res = int.Min(res, curr);
            }
            return res;
        }
    }
}