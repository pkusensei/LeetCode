using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string PushDominoes(string dominoes)
    {
        int[] arr = new int[dominoes.Length];
        int curr = 0;
        foreach (var (i, c) in dominoes.Select((c, i) => (i, c)))
        {
            curr = c switch
            {
                'R' => dominoes.Length,
                'L' => 0,
                _ => Math.Max(curr - 1, 0)
            };
            arr[i] = curr;
        }
        curr = 0;
        foreach (var (i, c) in dominoes.Select((c, i) => (i, c)).Reverse())
        {
            curr = c switch
            {
                'L' => dominoes.Length,
                'R' => 0,
                _ => Math.Max(curr - 1, 0)
            };
            arr[i] -= curr;
        }
        char[] chs = [.. arr.Select(v =>
        {
            if (v > 0) { return 'R'; }
            else if (v < 0) { return 'L'; }
            else { return '.'; }
        })];
        return new(chs);
    }
}

