using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FourSumCount(int[] nums1, int[] nums2, int[] nums3, int[] nums4)
    {
        Dictionary<int, int> freq = [];
        foreach (var v1 in nums1)
        {
            foreach (var v2 in nums2)
            {
                int key = -(v1 + v2);
                if (!freq.TryAdd(key, 1)) { freq[key] += 1; }
            }
        }
        int res = 0;
        foreach (var v1 in nums3)
        {
            foreach (var v2 in nums4)
            {
                res += freq.GetValueOrDefault(v1 + v2, 0);
            }
        }
        return res;
    }
}