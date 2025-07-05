using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindLucky(int[] arr)
    {
        Span<int> freq = stackalloc int[501];
        foreach (var item in arr)
        {
            freq[item] += 1;
        }
        int res = -1;
        for (int i = 1; i < 501; i++)
        {
            if (i == freq[i]) { res = i; }
        }
        return res;
    }
}
