using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int SubarrayBitwiseORs(int[] arr)
    {
        HashSet<int> res = [];
        HashSet<int> dp = [];
        foreach (var num in arr)
        {
            HashSet<int> curr = [num]; // start new subarr
            foreach (var item in dp)
            {
                curr.Add(item | num); // or num onto previous subarrs
            }
            dp = curr;
            res.UnionWith(dp); // ends subarrs here
        }
        return res.Count;
    }
}
