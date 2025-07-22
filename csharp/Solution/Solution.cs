using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindKthLargest(int[] nums, int k)
    {
        if (nums.Length == 0) { return 0; }
        int pivot = nums[^k];
        int[] left = [.. nums.Where(v => v > pivot)];
        int[] mid = [.. nums.Where(v => v == pivot)];
        int[] right = [.. nums.Where(v => v < pivot)];
        int n1 = left.Length;
        int n2 = mid.Length;
        if (k <= n1) { return FindKthLargest(left, k); }
        else if (n1 + n2 < k) { return FindKthLargest(right, k - n1 - n2); }
        else return mid[0];
    }
}