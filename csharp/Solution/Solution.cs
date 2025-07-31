using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] CountBits(int n)
    {
        int[] res = new int[1 + n];
        for (int i = 1; i <= n; i++)
        {
            res[i] = (i & 1) + res[i >> 1];
        }
        return res;
    }
}
