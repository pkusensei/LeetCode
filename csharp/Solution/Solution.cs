using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] SingleNumber(int[] nums)
    {
        int xor = nums.Aggregate((a, b) => a ^ b);
        int target_bit = 0;
        for (int i = 0; i < 32; i++)
        {
            if (((xor >> i) & 1) == 1)
            {
                target_bit = i;
                break;
            }
        }
        int a = 0;
        int b = 0;
        foreach (var num in nums)
        {
            if (((num >> target_bit) & 1) == 1) { a ^= num; }
            else { b ^= num; }
        }
        return [a, b];
    }
}