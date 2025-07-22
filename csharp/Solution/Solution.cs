using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<int>> CombinationSum3(int k, int n)
    {
        List<IList<int>> res = [];
        for (int mask = 0; mask < 1 << 10; mask++)
        {
            if ((mask & 1) == 0 && BitCount(mask) == k)
            {
                List<int> curr = [];
                int sum = 0;
                for (int bit = 1; bit <= 9; bit++)
                {
                    if (((mask >> bit) & 1) == 1)
                    {
                        sum += bit;
                        curr.Add(bit);
                    }
                }
                if (sum == n) { res.Add(curr); }
            }
        }
        return res;

        static int BitCount(int x)
        {
            int res = 0;
            while (x != 0)
            {
                x &= x - 1;
                res += 1;
            }
            return res;
        }
    }
}