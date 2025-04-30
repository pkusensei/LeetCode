using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long MinEnd(int n, int x)
    {
        var sb = new StringBuilder(Convert.ToString(x, 2), 64);
        var val = Convert.ToString(n - 1, 2);
        int i2 = val.Length - 1;
        for (int i1 = sb.Length - 1; i1 >= 0; i1 -= 1)
        {
            if (sb[i1] == '1') { continue; }
            sb[i1] = val[i2];
            i2 -= 1;
            if (i2 < 0) { break; }
        }
        while (i2 >= 0)
        {
            sb.Insert(0, val[i2]);
            i2 -= 1;
        }
        return Convert.ToInt64(sb.ToString(), 2);
    }
}

