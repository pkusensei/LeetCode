using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void SortColors(int[] nums)
    {
        Span<int> freq = stackalloc int[3];
        foreach (var item in nums)
        {
            freq[item] += 1;
        }
        for (int i = 0; i < nums.Length; i++)
        {
            if (freq[0] > 0) { nums[i] = 0; freq[0] -= 1; }
            else if (freq[1] > 0) { nums[i] = 1; freq[1] -= 1; }
            else if (freq[2] > 0) { nums[i] = 2; freq[2] -= 1; }
        }
    }
}