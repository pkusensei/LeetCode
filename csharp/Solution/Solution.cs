using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> KSmallestPairs(int[] nums1, int[] nums2, int k)
    {
        int n1 = nums1.Length;
        int n2 = nums2.Length;
        List<IList<int>> res = new(k);
        PriorityQueue<(int, int), int> pq = new();
        pq.Enqueue((0, 0), nums1[0] + nums2[0]);
        HashSet<(int, int)> seen = [(0, 0)];
        while (res.Count < k && pq.TryDequeue(out var item, out _))
        {
            (int i1, int i2) = item;
            res.Add([nums1[i1], nums2[i2]]);
            if (i1 + 1 < n1 && seen.Add((1 + i1, i2)))
            {
                pq.Enqueue((1 + i1, i2), nums1[1 + i1] + nums2[i2]);
            }
            if (1 + i2 < n2 && seen.Add((i1, 1 + i2)))
            {
                pq.Enqueue((i1, 1 + i2), nums1[i1] + nums2[1 + i2]);
            }
        }
        return res;
    }
}