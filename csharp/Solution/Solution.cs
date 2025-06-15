using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxDiff(int num)
    {
        List<int> ds = [];
        int _num = num;
        while (_num > 0)
        {
            ds.Add(_num % 10);
            _num /= 10;
        }
        ds.Reverse();
        int min = num;
        int max = num;
        for (int x = 0; x < 10; x++)
        {
            for (int y = 0; y < 10; y++)
            {
                var val = Change(x, y);
                if (val[0] > 0)
                {
                    int v = val.Aggregate((acc, d) => acc * 10 + d);
                    min = Math.Min(min, v);
                    max = Math.Max(max, v);
                }

            }
        }
        return max - min;

        List<int> Change(int x, int y) => [.. ds.Select(d => d == x ? y : d)];
    }
}
