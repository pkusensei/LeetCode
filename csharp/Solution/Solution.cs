using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public void WiggleSort(int[] nums)
    {
        int[] freq = new int[5001];
        foreach (var num in nums)
        {
            freq[num] += 1;
        }
        int fi = 5001;
        for (int i = 1; i < nums.Length; i += 2)
        {
            while (fi > 0 && freq[fi - 1] == 0)
            {
                fi -= 1;
            }
            nums[i] = fi - 1;
            freq[fi - 1] -= 1;
        }
        for (int i = 0; i < nums.Length; i += 2)
        {
            while (fi > 0 && freq[fi - 1] == 0)
            {
                fi -= 1;
            }
            nums[i] = fi - 1;
            freq[fi - 1] -= 1;
        }
    }
}
