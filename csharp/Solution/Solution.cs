using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> GetRow(int rowIndex)
    {
        List<int> res = [1];
        for (int _ = 0; _ < rowIndex; _++)
        {
            List<int> curr = [1];
            for (int i = 0; i < res.Count - 1; i++)
            {
                curr.Add(res[i] + res[1 + i]);
            }
            curr.Add(1);
            res = curr;
        }
        return res;
    }
}