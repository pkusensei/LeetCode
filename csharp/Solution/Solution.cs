using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] MaxNumber(int[] nums1, int[] nums2, int k)
    {
        int[] res = new int[k];
        for (int k1 = 0; k1 <= nums1.Length; k1++)
        {
            int k2 = int.Min(k - k1, nums2.Length);
            if (k1 + k2 != k) { continue; }
            var curr = Merge(Build(nums1, k1), Build(nums2, k2));
            if (curr.Zip(res, (a, b) => a.CompareTo(b)).FirstOrDefault(c => c != 0) > 0)
            {
                res = curr;
            }
        }
        return res;

        static int[] Build(ReadOnlySpan<int> nums, int k)
        {
            if (k <= 0) { return []; }
            Stack<int> st = [];
            for (int i = 0; i < nums.Length; i++)
            {
                while (st.TryPeek(out int top) && top < nums[i] && nums.Length - i > k - st.Count)
                {
                    st.Pop();
                }
                if (st.Count < k) { st.Push(nums[i]); }
            }
            return [.. st.Reverse()];
        }

        static int[] Merge(ReadOnlySpan<int> n1, ReadOnlySpan<int> n2)
        {
            List<int> res = new(n1.Length + n2.Length);
            while (!n1.IsEmpty && !n2.IsEmpty)
            {
                if (Compare(n1, n2) >= 0)
                {
                    res.Add(n1[0]);
                    n1 = n1[1..];
                }
                else
                {
                    res.Add(n2[0]);
                    n2 = n2[1..];
                }
            }
            res.AddRange(n1);
            res.AddRange(n2);
            return [.. res];
        }

        static int Compare(ReadOnlySpan<int> n1, ReadOnlySpan<int> n2)
        {
            int i1 = 0, i2 = 0;
            for (; i1 < n1.Length && i2 < n2.Length; i1 += 1, i2 += 1)
            {
                if (n1[i1] != n2[i2]) { return n1[i1].CompareTo(n2[i2]); }
            }
            while (i1 < n1.Length)
            {
                if (n1[i1] != n2[0]) { return n1[i1].CompareTo(n2[0]); }
                i1++;
            }
            while (i2 < n2.Length)
            {
                if (n1[0] != n2[i2]) { return n1[0].CompareTo(n2[i2]); }
                i2++;
            }
            return 0;
        }
    }
}
