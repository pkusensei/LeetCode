using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinSwap(int[] nums1, int[] nums2)
    {
        int n = nums1.Length;
        List<int> skip = new(n) { 0 };
        List<int> swap = new(n) { 1 };
        for (int i = 1; i < n; i++)
        {
            skip.Add(n);
            swap.Add(n);
            if (nums1[i - 1] < nums1[i] && nums2[i - 1] < nums2[i])
            {
                // Already in order, same skip ops
                skip[i] = skip[i - 1];
                // But if `i-1` is swapped, `i` must be swapped
                swap[i] = 1 + swap[i - 1];
            }
            if (nums1[i - 1] < nums2[i] && nums2[i - 1] < nums1[i])
            {
                // swap `i-1` but skip `i`
                skip[i] = int.Min(skip[i], swap[i - 1]);
                // skip `i-1` but swap `i`
                swap[i] = int.Min(swap[i], 1 + skip[i - 1]);
            }

        }
        return int.Min(skip[^1], swap[^1]);
    }

    public int TopDown(int[] nums1, int[] nums2)
    {
        int n = nums1.Length;
        int[,] memo = new int[n, 2];
        for (int i = 0; i < n; i++)
        {
            memo[i, 0] = -1;
            memo[i, 1] = -1;
        }
        return Dfs(0, 0, -1, -1);

        int Dfs(int i, int swap, int preva, int prevb)
        {
            if (i >= n) { return 0; }
            if (memo[i, swap] > -1) { return memo[i, swap]; }
            int res = n;
            if (preva < nums1[i] && prevb < nums2[i])
            {
                res = Dfs(1 + i, 0, nums1[i], nums2[i]);
            }
            if (preva < nums2[i] && prevb < nums1[i])
            {
                res = int.Min(res, 1 + Dfs(1 + i, 1, nums2[i], nums1[i]));
            }
            memo[i, swap] = res;
            return res;
        }
    }
}
