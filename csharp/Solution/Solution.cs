using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MissingNumber(int[] nums)
    {
        int n = nums.Length;
        int xor = Enumerable.Range(0, 1 + n).Aggregate((a, b) => a ^ b);
        int curr = nums.Aggregate((a, b) => a ^ b);
        return xor ^ curr;
    }
}