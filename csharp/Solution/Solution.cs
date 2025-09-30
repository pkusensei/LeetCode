using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int TriangularSum(int[] nums)
    {
        List<int> vals = [.. nums];
        while (vals.Count > 1)
        {
            for (int i = 0; i < vals.Count - 1; i++)
            {
                vals[i] = (vals[i] + vals[1 + i]) % 10;
            }
            vals.RemoveAt(vals.Count - 1);
        }
        return vals[0];
    }
}