using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int HIndex(int[] citations)
    {
        int n = citations.Length;
        int[] freq = new int[1 + n];
        foreach (var v in citations)
        {
            if (v < n) { freq[v] += 1; }
            else { freq[n] += 1; }
        }
        int suffix = 0;
        for (int i = n; i >= 0; i -= 1)
        {
            suffix += freq[i];
            if (suffix >= i) { return i; }
        }
        return 0;
    }
}