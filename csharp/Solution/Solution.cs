using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindNthDigit(int n)
    {
        long n_ = n;
        long step = 1;
        long count = 9;
        long start = 1;
        while (n_ > step * count)
        {
            n_ -= step * count;
            step += 1;
            count *= 10;
            start *= 10;
        }
        string num = (start + (n_ - 1) / step).ToString();
        return num[(int)((n_ - 1) % step)] - '0';
    }
}
