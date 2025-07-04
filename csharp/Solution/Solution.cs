using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public char KthCharacter(long k, int[] operations)
    {
        int count = 0;
        for (int i = operations.Length - 1; i >= 0; i -= 1)
        {
            // half_len = pow(2, i)
            if (Math.Ceiling(Math.Log2(k)) >= i && k > Math.Pow(2, i))
            {
                k -= (long)Math.Pow(2, i);
                count += operations[i];
            }
        }
        return (char)('a' + count % 26);
    }
}
