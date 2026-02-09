using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] AdvantageCount(int[] nums1, int[] nums2)
    {
        int n = nums1.Length;
        Array.Sort(nums1);
        int[] ids = [.. Enumerable.Range(0, n)];
        Array.Sort(ids, (a, b) => nums2[a].CompareTo(nums2[b]));
        int[] res = new int[n];
        Array.Fill(res, -1);
        Stack<int> st = [];
        int i2 = 0;
        foreach (var num in nums1)
        {
            if (num > nums2[ids[i2]])
            {
                res[ids[i2]] = num;
                i2 += 1;
            }
            else { st.Push(num); }
        }
        for (int i = 0; i < n; i++)
        {
            if (res[i] == -1) { res[i] = st.Pop(); }
        }
        return res;
    }
}