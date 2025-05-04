using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumEquivDominoPairs(int[][] dominoes)
    {
        Dictionary<(int, int), int> dict = [];
        int res = 0;
        foreach (var item in dominoes)
        {
            int a = item.Min();
            int b = item.Max();
            if (dict.TryGetValue((a, b), out var v)) { res += v; }
            if (!dict.TryAdd((a, b), 1)) { dict[(a, b)] += 1; }
        }
        return res;
    }
}
