using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
public string PushDominoes(string dominoes)
{
    int n = dominoes.Length;
    int[] arr = new int[n];
    int curr = 0;
    for (int i = 0; i < n; i++)
    {
        curr = dominoes[i] switch
        {
            'R' => n,
            'L' => 0,
            _ => int.Max(0, curr - 1)
        };
        arr[i] = curr;
    }
    curr = 0;
    for (int i = n - 1; i >= 0; i -= 1)
    {
        curr = dominoes[i] switch
        {
            'L' => n,
            'R' => 0,
            _ => int.Max(0, curr - 1)
        };
        arr[i] -= curr;
    }
    return new([.. arr.Select(v => v switch
    {
        > 0 => 'R',
        < 0 => 'L',
        _ => '.'
    })]);
}
}
