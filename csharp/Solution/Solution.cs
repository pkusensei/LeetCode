using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] BeautifulArray(int n)
    {
        int[] res = [1];
        while (res.Length < n)
        {
            res = [.. res.Select(v => v * 2 - 1).Concat(res.Select(v => v * 2)).Where(v => v <= n)];
        }
        return res;
    }
}