using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool SplitArraySameAverage(int[] nums)
    {
        int n = nums.Length;
        if (n == 1) { return false; }
        int sum = nums.Sum();
        bool possible = false;
        for (int k = 1; k <= n / 2; k++)
        {
            if (sum * k % n == 0)
            {
                possible = true;
                break;
            }
        }
        if (!possible) { return false; }
        int[] arr = [.. nums.Select(v => v * n - sum)];
        int mid = n / 2;
        ReadOnlySpan<int> left = arr.AsSpan()[..mid];
        ReadOnlySpan<int> right = arr.AsSpan()[mid..];
        HashSet<int> left_sums = new(1 << mid) { 0 };
        foreach (var item in left)
        {
            List<int> next = new(1 << mid);
            foreach (var num in left_sums)
            {
                int ns = num + item;
                if (ns == 0) { return true; }
                next.Add(ns);
            }
            left_sums.UnionWith(next);
        }
        left_sums.Remove(0);
        HashSet<int> right_sums = new(1 << (n - mid)) { 0 };
        int total_right = arr.Skip(mid).Sum();
        foreach (var item in right)
        {
            List<int> next = [];
            foreach (var num in right_sums)
            {
                int ns = item + num;
                if (ns == 0) { return true; }
                if (left_sums.Contains(-ns))
                {
                    if (ns != total_right) { return true; }
                }
                next.Add(ns);
            }
            right_sums.UnionWith(next);
        }
        return false;
    }
}
