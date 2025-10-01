using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumWaterBottles(int numBottles, int numExchange)
    {
        int res = 0;
        int empty = 0;
        while (numBottles > 0)
        {
            res += numBottles;
            empty += numBottles;
            numBottles = empty / numExchange;
            empty %= numExchange;
        }
        return res;
    }
}