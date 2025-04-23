using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountLargestGroup(int n)
    {
        Dictionary<int, int> dict = [];
        for (int i = 1; i <= n; i++)
        {
            int sum = SumDigit(i);
            if (!dict.TryAdd(sum, 1)) { dict[sum] += 1; }
        }
        int max = dict.Values.Max();
        return dict.Values.Where(v => v == max).Count();

        static int SumDigit(int val)
        {
            int res = 0;
            while (val > 0)
            {
                res += val % 10;
                val /= 10;
            }
            return res;
        }
    }
}
