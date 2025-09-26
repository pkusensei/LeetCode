using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaximumSwap(int num)
    {
        char[] chs = num.ToString().ToCharArray();
        Span<int> last = stackalloc int[10];
        int n = chs.Length;
        for (int i = 0; i < n; i++)
        {
            last[chs[i] - '0'] = i;
        }
        for (int left = 0; left < n; left++)
        {
            int left_d = chs[left] - '0';
            for (int right_d = 9; right_d > left_d; right_d -= 1)
            {
                if (left < last[right_d])
                {
                    int right = last[right_d];
                    (chs[left], chs[right]) = (chs[right], chs[left]);
                    return int.Parse(chs);
                }
            }
        }
        return num;
    }
}