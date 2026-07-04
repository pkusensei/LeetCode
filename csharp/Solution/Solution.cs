using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] MaxDepthAfterSplit(string seq)
    {
        List<int> res = new(seq.Length);
        int open0 = 0;
        int open1 = 0;
        foreach (var item in seq)
        {
            if (item == '(')
            {
                if (open0 <= open1)
                {
                    res.Add(0);
                    open0 += 1;
                }
                else
                {
                    res.Add(1);
                    open1 += 1;
                }
            }
            else
            {
                if (open0 > open1)
                {
                    res.Add(0);
                    open0 -= 1;
                }
                else
                {
                    res.Add(1);
                    open1 -= 1;
                }
            }
        }
        return [.. res];
    }
}
