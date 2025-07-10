using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> PermuteUnique(int[] nums)
    {
        Dictionary<int, int> freq = nums.CountBy(_ => _).ToDictionary();
        List<IList<int>> res = [];
        Backtrack([]);
        return res;

        void Backtrack(List<int> curr)
        {
            if (curr.Count == nums.Length) { res.Add([.. curr]); }
            foreach (var (k, v) in freq)
            {
                if (v == 0) { continue; }
                freq[k] -= 1;
                curr.Add(k);
                Backtrack(curr);
                curr.RemoveAt(curr.Count - 1);
                freq[k] += 1;
            }
        }
    }
}
