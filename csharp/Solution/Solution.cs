using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class FindSumPairs
{
    int[] Nums2 { get; init; }
    Dictionary<int, int> Dict1 { get; init; }
    Dictionary<int, int> Dict2 { get; init; }

    public FindSumPairs(int[] nums1, int[] nums2)
    {
        Nums2 = nums2;
        Dict1 = [];
        Dict2 = [];
        foreach (var item in nums1)
        {
            if (!Dict1.TryAdd(item, 1)) { Dict1[item] += 1; }
        }
        foreach (var item in nums2)
        {
            if (!Dict2.TryAdd(item, 1)) { Dict2[item] += 1; }
        }
    }

    public void Add(int index, int val)
    {
        int oldval = Nums2[index];
        Dict2[oldval] -= 1;
        Nums2[index] += val;
        if (!Dict2.TryAdd(Nums2[index], 1)) { Dict2[Nums2[index]] += 1; }
    }

    public int Count(int tot)
    {
        int res = 0;
        foreach (var (n1, v1) in Dict1)
        {
            res += v1 * Dict2.GetValueOrDefault(tot - n1, 0);
        }
        return res;
    }
}
