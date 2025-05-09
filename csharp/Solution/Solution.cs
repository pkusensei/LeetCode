using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumberOfAlternatingGroups(int[] colors, int k)
    {
        int curr = 1;
        int res = 0;
        int n = colors.Length;
        for (int i = 1; i < n + k - 1; i++)
        {
            if (colors[i % n] != colors[(i - 1) % n]) { curr += 1; }
            else { curr = 1; }
            res += curr >= k ? 1 : 0;
        }
        return res;
    }
}
