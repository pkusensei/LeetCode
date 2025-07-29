using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountRangeSum(int[] nums, int lower, int upper)
    {
        List<long> prefix = [0]; // [0] so we can query before start of array
        foreach (var num in nums)
        {
            prefix.Add(prefix[^1] + num);
        }
        SortedSet<long> values = [.. prefix.SelectMany(v => new[] { v, v - lower, v - upper })];
        Dictionary<long, int> compressed = [];
        foreach (var val in values)
        {
            compressed.Add(val, compressed.Count);
        }
        BIT tree = new(values.Count);
        int res = 0;
        // lower <= sum(i1..=i2) <= upper
        // lower <= ps(1+i2) - ps(i1) <= upper // ps starts with [0]
        // ps(1+i2) - lower >= ps(i) >= ps(1+i2) - upper
        foreach (var item in prefix)
        {
            int left = compressed[item - upper];
            int right = compressed[item - lower];
            res += tree.RangeQuery(left, right);
            tree.Update(compressed[item], 1);
        }
        return res;
    }

    public int WithMergeSort(int[] nums, int lower, int upper)
    {
        long[] prefix = new long[1 + nums.Length];
        for (int i = 0; i < nums.Length; i++)
        {
            prefix[1 + i] = prefix[i] + nums[i];
        }
        return MergeSort(prefix);

        int MergeSort(Span<long> prefix)
        {
            int mid = prefix.Length / 2;
            if (mid == 0) { return 0; }
            int res = MergeSort(prefix[..mid]) + MergeSort(prefix[mid..]);
            int left = mid;
            int right = mid;
            foreach (var val in prefix[..mid]) // Fix one prefix sum
            {
                while (left < prefix.Length && prefix[left] - val < lower)
                {
                    left += 1;
                }
                while (right < prefix.Length && prefix[right] - val <= upper)
                {
                    right += 1;
                }
                res += right - left; // All in this range have diff in [lower, upper]
            }
            prefix.Sort();
            return res;
        }
    }
}

internal class BIT
{
    public BIT(int n)
    {
        Tree = new int[1 + n];
        N = n;
    }

    int[] Tree { get; }
    int N { get; }

    public void Update(int idx, int val)
    {
        idx += 1;
        while (idx <= N)
        {
            Tree[idx] += val;
            idx += idx & -idx;
        }
    }

    public int Query(int idx)
    {
        idx += 1;
        int res = 0;
        while (idx > 0)
        {
            res += Tree[idx];
            idx -= idx & -idx;
        }
        return res;
    }

    public int RangeQuery(int left, int right) => Query(right) - Query(left - 1);
}
