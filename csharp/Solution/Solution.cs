using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxSatisfied(int[] customers, int[] grumpy, int minutes)
    {
        int curr = 0;
        int n = customers.Length;
        for (int i = 0; i < n; i++)
        {
            if (i < minutes) { curr += customers[i]; }
            else { curr += (1 - grumpy[i]) * customers[i]; }
        }
        int res = curr;
        for (int i = minutes; i < n; i++)
        {
            curr += grumpy[i] * customers[i];
            curr -= grumpy[i - minutes] * customers[i - minutes];
            res = int.Max(res, curr);
        }
        return res;
    }
}
