using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long KthSmallestProduct(int[] nums1, int[] nums2, long k)
    {
        (var a1, var a2) = Prep(nums1);
        (var b1, var b2) = Prep(nums2);
        long neg_count = (long)a1.Count * b2.Count + (long)a2.Count * b1.Count;
        if (k <= neg_count) { (b1, b2) = (b2, b1); }
        int sign = k > neg_count ? 1 : -1;
        k = k > neg_count ? k - neg_count : neg_count + 1 - k;
        long left = 0;
        long right = 10_000_000_001;
        while (left < right)
        {
            var mid = left + (right - left) / 2;
            if (Count(a1, b1, mid) + Count(a2, b2, mid) >= k) { right = mid; }
            else { left = 1 + mid; }
        }
        return sign * left;

        static long Count(IList<long> arr1, IList<long> arr2, long mid)
        {
            long res = 0;
            int i2 = arr2.Count;
            foreach (var v1 in arr1)
            {
                while (i2 > 0 && v1 * arr2[i2 - 1] > mid) { i2 -= 1; }
                res += i2;
            }
            return res;
        }

        static (List<long>, List<long>) Prep(int[] nums)
        {
            List<long> v1 = [];
            List<long> v2 = [];
            foreach (var num in nums)
            {
                if (num < 0) { v1.Add(-num); }
                else { v2.Add(num); }
            }
            v1.Reverse();
            return (v1, v2);
        }
    }
}
