using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxBottlesDrunk(int numBottles, int numExchange)
    {
        int res = 0;
        int empty = 0;
        while (numBottles > 0)
        {
            res += numBottles;
            empty += numBottles;
            numBottles = 0;
            while (empty >= numExchange)
            {
                empty -= numExchange;
                numExchange += 1;
                numBottles += 1;
            }
        }
        return res;
    }
}