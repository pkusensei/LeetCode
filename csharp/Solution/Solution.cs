using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> Permute(int[] nums)
    {
        List<IList<int>> res = [];
        Backtrack(nums);
        return res;

        void Backtrack(Span<int> span)
        {
            if (span.IsEmpty) { res.Add([.. nums]); }
            for (int i = 0; i < span.Length; i++)
            {
                (span[0], span[i]) = (span[i], span[0]);
                Backtrack(span[1..]);
                (span[0], span[i]) = (span[i], span[0]);
            }
        }
    }
}
